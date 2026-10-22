use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use crate::domain::{
    DecisionEngine, InputType, IntakeDecisionResult, Priority, RiskLane, SystemOneQuestion,
    SystemOneRequest, SystemOneResponse, WorkItemType,
};

pub struct JevClient {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub model: String,
    pub fallback_endpoint: Option<String>,
    pub fallback_model: Option<String>,
    pub rate_limit_ms: u64,
    last_call: Mutex<Option<Instant>>,
}

impl JevClient {
    pub fn from_env() -> Self {
        let mut endpoint = env::var("HARNESS_JEV_ENDPOINT").ok();
        let mut model = env::var("HARNESS_JEV_MODEL").ok();
        let api_key = env::var("HARNESS_JEV_API_KEY").ok();
        let fallback_endpoint = env::var("HARNESS_JEV_FALLBACK_ENDPOINT").ok();
        let fallback_model = env::var("HARNESS_JEV_FALLBACK_MODEL").ok();
        let rate_limit_ms = env::var("HARNESS_JEV_RATE_LIMIT_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(200);

        if endpoint.is_none() || model.is_none() {
            for candidate in &[".env", "../.env"] {
                if let Ok(content) = std::fs::read_to_string(candidate) {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.is_empty() || trimmed.starts_with('#') {
                            continue;
                        }
                        if let Some((k, v)) = trimmed.split_once('=') {
                            let k = k.trim();
                            let v = v.trim().trim_matches('"').trim_matches('\'').to_string();
                            if k == "HARNESS_JEV_ENDPOINT" && endpoint.is_none() {
                                endpoint = Some(v);
                            } else if k == "HARNESS_JEV_MODEL" && model.is_none() {
                                model = Some(v);
                            }
                        }
                    }
                }
            }
        }

        let endpoint =
            endpoint.unwrap_or_else(|| "http://100.124.36.46:8008/v1/systemone".to_string());
        let model = model.unwrap_or_else(|| "kev-latest".to_string());

        Self {
            endpoint,
            api_key,
            model,
            fallback_endpoint,
            fallback_model,
            rate_limit_ms,
            last_call: Mutex::new(None),
        }
    }

    fn throttle(&self) {
        let mut last = self.last_call.lock().unwrap();
        if let Some(prev) = *last {
            let elapsed = prev.elapsed();
            let limit = Duration::from_millis(self.rate_limit_ms);
            if elapsed < limit {
                thread::sleep(limit - elapsed);
            }
        }
        *last = Some(Instant::now());
    }

    pub fn build_intake_request(&self, spec_text: &str, model: &str) -> SystemOneRequest {
        let mut questions = HashMap::new();

        // 1. input_type question
        let mut input_criteria = HashMap::new();
        input_criteria.insert(
            "new_spec".to_string(),
            "Đặc tả toàn bộ hệ thống hoặc dự án mới".to_string(),
        );
        input_criteria.insert(
            "spec_slice".to_string(),
            "Triển khai một phần tính năng đã có trong spec lớn".to_string(),
        );
        input_criteria.insert(
            "change_request".to_string(),
            "Thay đổi, sửa đổi hoặc tinh chỉnh hành vi hiện tại".to_string(),
        );
        input_criteria.insert(
            "new_initiative".to_string(),
            "Vùng tính năng lớn cần nhiều story".to_string(),
        );
        input_criteria.insert(
            "maintenance".to_string(),
            "Bảo trì kỹ thuật, nâng cấp thư viện, hiệu năng".to_string(),
        );
        input_criteria.insert(
            "harness_improvement".to_string(),
            "Cải tiến quy trình, template hoặc tool harness".to_string(),
        );

        questions.insert(
            "input_type".to_string(),
            SystemOneQuestion::Choice {
                instructions: "Loại công việc của yêu cầu này là gì?".to_string(),
                criteria: input_criteria,
            },
        );

        // 2. predicted_lane question
        let mut lane_criteria = HashMap::new();
        lane_criteria.insert(
            "tiny".to_string(),
            "Rủi ro thấp, chỉ sửa docs, copy, tên hoặc sửa đổi hẹp".to_string(),
        );
        lane_criteria.insert(
            "normal".to_string(),
            "Tính năng kích thước story có phạm vi ảnh hưởng giới hạn".to_string(),
        );
        lane_criteria.insert(
            "high_risk".to_string(),
            "Ảnh hưởng auth, data model, bảo mật, hợp đồng API công khai".to_string(),
        );

        questions.insert(
            "predicted_lane".to_string(),
            SystemOneQuestion::Choice {
                instructions: "Đánh giá mức độ rủi ro (Risk Lane) của tác vụ?".to_string(),
                criteria: lane_criteria,
            },
        );

        // 3. priority question
        let mut priority_criteria = HashMap::new();
        priority_criteria.insert(
            "p0".to_string(),
            "Khẩn cấp, ảnh hưởng trực tiếp hệ thống hoặc chặn luồng release".to_string(),
        );
        priority_criteria.insert(
            "p1".to_string(),
            "Ưu tiên cao, tính năng cốt lõi của Sprint hiện tại".to_string(),
        );
        priority_criteria.insert(
            "p2".to_string(),
            "Bình thường, theo kế hoạch định kỳ".to_string(),
        );
        priority_criteria.insert(
            "p3".to_string(),
            "Ưu tiên thấp, có thể dời sang Sprint tiếp theo".to_string(),
        );

        questions.insert(
            "priority".to_string(),
            SystemOneQuestion::Choice {
                instructions: "Mức độ ưu tiên xử lý của yêu cầu?".to_string(),
                criteria: priority_criteria,
            },
        );

        // 4. work_item_type question
        let mut item_criteria = HashMap::new();
        item_criteria.insert(
            "epic".to_string(),
            "Sáng kiến chiến lược lớn bao gồm nhiều phân hệ hoặc kéo dài nhiều Sprint".to_string(),
        );
        item_criteria.insert(
            "feature".to_string(),
            "Tính năng lớn mang lại giá trị nghiệp vụ, gồm nhiều User Stories".to_string(),
        );
        item_criteria.insert(
            "user_story".to_string(),
            "Yêu cầu chức năng cụ thể có thể hoàn thành và kiểm thử trong 1 Sprint".to_string(),
        );
        item_criteria.insert(
            "technical_story".to_string(),
            "Công việc kỹ thuật, refactor, tối ưu hạ tầng, CI/CD".to_string(),
        );
        item_criteria.insert(
            "task".to_string(),
            "Nhiệm vụ kỹ thuật con trực thuộc một Story".to_string(),
        );
        item_criteria.insert(
            "bug".to_string(),
            "Lỗi sai lệch so với tài liệu đặc tả cần khắc phục".to_string(),
        );

        questions.insert(
            "work_item_type".to_string(),
            SystemOneQuestion::Choice {
                instructions: "Cấp độ phân tầng Work Item chuẩn cho yêu cầu này là gì?".to_string(),
                criteria: item_criteria,
            },
        );

        // 5. is_urgent question
        questions.insert(
            "is_urgent".to_string(),
            SystemOneQuestion::Noul {
                instructions: "Tác vụ này có phải sự cố khẩn cấp cần xử lý ngay không?".to_string(),
            },
        );

        SystemOneRequest {
            model: model.to_string(),
            state: spec_text.to_string(),
            questions,
        }
    }

    fn post_systemone(
        &self,
        endpoint: &str,
        request: &SystemOneRequest,
    ) -> Result<SystemOneResponse, String> {
        self.throttle();

        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(10))
            .build();

        let mut req = agent.post(endpoint).set("Content-Type", "application/json");

        if let Some(ref key) = self.api_key {
            req = req.set("Authorization", &format!("Bearer {}", key));
        }

        let resp = req
            .send_json(request)
            .map_err(|e| format!("HTTP request to {} failed: {}", endpoint, e))?;

        let body_str = resp
            .into_string()
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        let parsed: SystemOneResponse = serde_json::from_str(&body_str).map_err(|e| {
            format!(
                "Failed to parse SystemOneResponse from JSON: {}\nRaw Body: {}",
                e, body_str
            )
        })?;

        Ok(parsed)
    }

    pub fn call_request(&self, request: &SystemOneRequest) -> Result<SystemOneResponse, String> {
        let mut last_err = String::new();
        for attempt in 1..=3 {
            match self.post_systemone(&self.endpoint, request) {
                Ok(resp) => return Ok(resp),
                Err(e) => {
                    last_err = e;
                    if attempt < 3 {
                        thread::sleep(Duration::from_millis(200 * attempt));
                    }
                }
            }
        }

        // Try Fallback endpoint if available
        if let Some(ref fallback_ep) = self.fallback_endpoint {
            let mut fb_req = request.clone();
            if let Some(ref fb_model) = self.fallback_model {
                fb_req.model = fb_model.clone();
            }
            if let Ok(resp) = self.post_systemone(fallback_ep, &fb_req) {
                return Ok(resp);
            }
        }

        Err(format!(
            "SystemOne Decision Model call failed after retries. Last error: {}",
            last_err
        ))
    }

    pub fn call_with_retry_and_fallback(
        &self,
        spec_text: &str,
    ) -> Result<SystemOneResponse, String> {
        let req_primary = self.build_intake_request(spec_text, &self.model);
        self.call_request(&req_primary)
    }
}

impl DecisionEngine for JevClient {
    fn evaluate_intake(&self, spec_text: &str) -> Result<IntakeDecisionResult, String> {
        let response = self.call_with_retry_and_fallback(spec_text)?;

        let mut input_type = InputType::ChangeRequest;
        let mut predicted_lane = RiskLane::Normal;
        let mut priority = Priority::P2;
        let mut work_item_type = None;
        let mut is_urgent = false;
        let mut confidence = 0.5;
        let mut probabilities = HashMap::new();

        if let Some(ans) = response.answers.get("input_type") {
            if let Some(ref choice) = ans.choice {
                if let Ok(val) = InputType::from_str(choice) {
                    input_type = val;
                }
            }
            if let Some(conf) = ans.confidence {
                confidence = conf;
            }
            if let Some(ref probs) = ans.probabilities {
                probabilities.extend(probs.clone());
            }
        }

        if let Some(ans) = response.answers.get("predicted_lane") {
            if let Some(ref choice) = ans.choice {
                if let Ok(val) = RiskLane::from_str(choice) {
                    predicted_lane = val;
                }
            }
        }

        if let Some(ans) = response.answers.get("priority") {
            if let Some(ref choice) = ans.choice {
                if let Ok(val) = Priority::from_str(choice) {
                    priority = val;
                }
            }
        }

        if let Some(ans) = response.answers.get("work_item_type") {
            if let Some(ref choice) = ans.choice {
                if let Ok(val) = WorkItemType::from_str(choice) {
                    work_item_type = Some(val);
                }
            }
        }

        if let Some(ans) = response.answers.get("is_urgent") {
            if let Some(noul_val) = ans.noul {
                is_urgent = noul_val >= 0.5;
            }
        }

        let summary_reason = format!(
            "Classified by JEV ({}) with confidence {:.2}",
            response.model, confidence
        );

        Ok(IntakeDecisionResult {
            input_type,
            predicted_lane,
            priority,
            work_item_type,
            is_urgent,
            confidence,
            probabilities,
            summary_reason,
        })
    }
}
