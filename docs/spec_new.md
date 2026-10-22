# QUY TRÌNH PHÁT TRIỂN & PHÁT HÀNH SẢN PHẨM PHẦN MỀM (FULL SPECIFICATION MANUAL) 🚀

> **CÔNG TY (Company)**  
> **Địa chỉ**: Tầng 2,4 Lô III-26, Đường 19/5A, Nhóm CN III, Khu công nghiệp Tân Bình, Tân Phú, TPHCM  
> **Mã số quy trình hợp nhất**: SM-QT-01 | SM-QT-02 | SM-QT-03 | SM-QT-04 | SM-QĐi-01 | SM-QĐ-002 | DDD-COMPANY-2026  
> **Ngày hiệu lực**: 2024 – 2026 (Phiên bản v1.0 / v1.1 - Cập nhật 26/06/2026)  
> **Tiêu chuẩn áp dụng**: Domain-Driven Design (DDD) | Microservices Architecture | ISO/IEC 27001:2022 | Scrum Framework 2020  

---

## 📋 MỤC LỤC TỔNG QUAN

1. [QUY TRÌNH 1: TIẾP NHẬN YÊU CẦU VÀ LẬP KẾ HOẠCH PHÁT TRIỂN SẢN PHẨM (SM-QT-01)](#1-quy-trình-1-tiếp-nhận-yêu-cầu-và-lập-kế-hoạch-phát-triển-sản-phẩm-sm-qt-01)
   - 1.1 [Mục Đích & Phạm Vi Áp Dụng](#11-mục-đích--phạm-vi-áp-dụng)
   - 1.2 [Trách Nhiệm Thực Hiện](#12-trách-nhiệm-thực-hiện)
   - 1.3 [Thuật Ngữ & Định Nghĩa](#13-thuật-ngữ--định-nghĩa)
   - 1.4 [Sơ Đồ & Các Bước Thực Hiện Chi Tiết](#14-sơ-đồ--các-bước-thực-hiện-chi-tiết)
   - 1.5 [Danh Mục Hồ Sơ Lưu Trữ (SM-QT-01)](#15-danh-mục-hồ-sơ-lưu-trữ-sm-qt-01)
2. [QUY TRÌNH 2: THỰC HIỆN PHÁT TRIỂN PHẦN MỀM THEO SCRUM FRAMEWORK (SM-QT-02)](#2-quy-trình-2-thực-hiện-phát-triển-phần-mềm-theo-scrum-framework-sm-qt-02)
   - 2.1 [Mục Đích & Phạm Vi Áp Dụng](#21-mục-đích--phạm-vi-áp-dụng)
   - 2.2 [Trách Nhiệm Của Các Vai Trò Trong Scrum](#22-trách-nhiệm-của-các-vai-trò-trong-scrum)
   - 2.3 [Thuật Ngữ & Định Nghĩa Cốt Lõi (Bao gồm DoR & Estimation)](#23-thuật-ngữ--định-nghĩa-cốt-lõi)
   - 2.4 [Sơ Đồ & Chi Tiết Các Sự Kiện Sprint (Sprint Events)](#24-sơ-đồ--chi-tiết-các-sự-kiện-sprint-sprint-events)
   - 2.5 [Danh Mục Hồ Sơ Lưu Trữ (SM-QT-02)](#25-danh-mục-hồ-sơ-lưu-trữ-sm-qt-02)
3. [QUY TRÌNH 3: PHÁT HÀNH SẢN PHẨM PHẦN MỀM - RELEASE (SM-QT-03)](#3-quy-trình-3-phát-hành-sản-phẩm-phần-mềm---release-sm-qt-03)
   - 3.1 [Mục Đích & Phạm Vi Áp Dụng](#31-mục-đích--phạm-vi-áp-dụng)
   - 3.2 [Trách Nhiệm Thực Hiện Phát Hành](#32-trách-nhiệm-thực-hiện-phát-hành)
   - 3.3 [Thuật Ngữ & Định Nghĩa](#33-thuật-ngữ--định-nghĩa)
   - 3.4 [Sơ Đồ & Luồng Triển Khai Phát Hành Chi Tiết](#34-sơ-đồ--luồng-triển-khai-phát-hành-chi-tiết)
   - 3.5 [Quy Định Khung Thời Gian & Phê Duyệt Release](#35-quy-định-khung-thời-gian--phê-duyệt-release)
   - 3.6 [Danh Mục Hồ Sơ Lưu Trữ (SM-QT-03)](#36-danh-mục-hồ-sơ-lưu-trữ-sm-qt-03)
4. [QUY TRÌNH 4: XỬ LÝ YÊU CẦU BỔ SUNG TRONG MÔ HÌNH MICROSERVICES (SM-QT-04)](#4-quy-trình-4-xử-lý-yêu-cầu-bổ-sung-trong-mô-hình-microservices-sm-qt-04)
   - 4.1 [Mục Đích & Nguyên Tắc Phối Hợp Provider - Tenant](#41-mục-đích--nguyên-tắc-phối-hợp-provider---tenant)
   - 4.2 [Trách Nhiệm Các Bên & SLA Tiếp Nhận (24h)](#42-trách-nhiệm-các-bên--sla-tiếp-nhận-24h)
   - 4.3 [Thuật Ngữ & Định Nghĩa](#43-thuật-ngữ--định-nghĩa)
   - 4.4 [Biểu Mẫu Output Request Form (ORF)](#44-biểu-mẫu-output-request-form-orf)
   - 4.5 [Sơ Đồ & Luồng Xử Lý Cross-Team Chi Tiết](#45-sơ-đồ--luồng-xử-lý-cross-team-chi-tiết)
   - 4.6 [Danh Mục Hồ Sơ Lưu Trữ (SM-QT-04)](#46-danh-mục-hồ-sơ-lưu-trữ-sm-qt-04)
5. [QUY ĐỊNH 5: QUẢN LÝ BACKLOG VÀ WORK ITEMS TRÊN TFS / JIRA (SM-QĐi-01)](#5-quy-định-5-quản-lý-backlog-và-work-items-trên-tfs--jira-sm-qđi-01)
   - 5.1 [Mục Đích & Phạm Vi Áp Dụng](#51-mục-đích--phạm-vi-áp-dụng)
   - 5.2 [Cấu Trúc Phân Tầng Work Items (Portfolio Backlog ➔ Issue Tracking)](#52-cấu-trúc-phân-tầng-work-items-portfolio-backlog--issue-tracking)
   - 5.3 [Thành Phần Chi Tiết & Quy Định Các Trường Dữ Liệu](#53-thành-phần-chi-tiết--quy-định-các-trường-dữ-liệu)
   - 5.4 [Luồng Trạng Thái (State Machine) Của Từng Work Item](#54-luồng-trạng-thái-state-machine-của-từng-work-item)
   - 5.5 [Quy Ước Chuẩn Viết Nội Dung Cho Epic, Feature, User Story, Task, Bug, Testcase](#55-quy-ước-chuẩn-viết-nội-dung-cho-epic-feature-user-story-task-bug-testcase)
   - 5.6 [Danh Mục Hồ Sơ Lưu Trữ (SM-QĐi-01)](#56-danh-mục-hồ-sơ-lưu-trữ-sm-qđi-01)
6. [QUY ĐỊNH 6: QUẢN LÝ WORK ITEM TRÊN TFS, CHIẾN LƯỢC GIT BRANCH & CI/CD (SM-QĐ-002 - HIỆU LỰC 26/06/2026)](#6-quy-định-6-quản-lý-work-item-trên-tfs-sm-qđ-002---hiệu-lực-26062026)
   - 6.1 [Mục Đích & Phạm Vi Áp Dụng](#61-mục-đích--phạm-vi-áp-dụng-1)
   - 6.2 [Trách Nhiệm Theo Vai Trò (CPO, BA, Dev, Tester, SM/PM)](#62-trách-nhiệm-theo-vai-trò-cpo-ba-dev-tester-smpm)
   - 6.3 [Định Nghĩa Các Loại Work Item & Technical Story](#63-định-nghĩa-các-loại-work-item--technical-story)
   - 6.4 [Ma Trận Trạng Thái & Người Chuyển Trạng Thái](#64-ma-trận-trạng-thái--người-chuyển-trạng-thái)
   - 6.5 [Phân Định Môi Trường, Flow Deploy Code, Branching & CI/CD Pipeline](#65-phân-định-môi-trường-phát-triển--flow-deploy-code)
   - 6.6 [Workflow Chi Tiết Từng Loại Work Item](#66-workflow-chi-tiết-từng-loại-work-item)
7. [KIẾN TRÚC & THIẾT KẾ DOMAIN-DRIVEN DESIGN (DDD) VÀ THIẾT LẬP MÔ HÌNH TEAM OWNERSHIP TẠI COMPANY](#7-kiến-trúc--thiết-kế-domain-driven-design-ddd-và-thiết-lập-mô-hình-team-ownership-tại-company)
   - 7.1 [Tổng Quan Kiến Trúc Microservices tại Company](#71-tổng-quan-kiến-trúc-microservices-tại-company)
   - 7.2 [Thiết Kế Hệ Thống Theo Miền Nghiệp Vụ (Domain & Bounded Context)](#72-thiết-kế-hệ-thống-theo-miền-nghiệp-vụ-domain--bounded-context)
   - 7.3 [Cấu Trúc Phân Tầng Kiến Trúc (Architecture Layers Topology)](#73-cấu-trúc-phân-tầng-kiến-trúc-architecture-layers-topology)
   - 7.4 [Thiết Lập Mô Hình Team Ownership & Domain Ownership](#74-thiết-lập-mô-hình-team-ownership--domain-ownership)
   - 7.5 [Nguyên Tắc Giao Tiếp & Minh Bạch Thông Tin Giữa Các Nhóm](#75-nguyên-tắc-giao-tiếp--minh-bạch-thông-tin-giữa-các-nhóm)
   - 7.6 [Quản Lý Domain Knowledge & Lưu Trữ Tài Liệu Trực Tuyến](#76-quản-lý-domain-knowledge--lưu-trữ-tài-liệu-trực-tuyến)
   - 7.7 [Quy Định Về UI Front-End, UI Widgets & UI Fragments Dùng Chung](#77-quy-định-về-ui-front-end-ui-widgets--ui-fragments-dùng-chung)
   - 7.8 [Phân Định Trách Nhiệm Kiểm Thử (Product Testing vs Domain Testing)](#78-phân-định-trách-nhiệm-kiểm-thử-product-testing-vs-domain-testing)
   - 7.9 [Quy Trình Xử Lý Thay Đổi Phá Vỡ (Breaking Change)](#79-quy-trình-xử-lý-thay-đổi-phá-vỡ-breaking-change)
   - 7.10 [Phân Bổ Tỷ Lệ Ưu Tiên Product Backlog & Quyền Quyết Định Business Rule](#710-phân-bổ-tỷ-lệ-ưu-tiên-product-backlog--quyền-quyết-định-business-rule)
   - 7.11 [Phân Định Quyền Hạn Chi Tiết (Product Team vs Domain Team vs CPO)](#711-phân-định-quyền-hạn-chi-tiết-product-team-vs-domain-team-vs-cpo)
   - 7.12 [Ma Trận Phân Công Công Việc Tích Hợp Thực Tế (Ví dụ: TPOS tích hợp Loyalty)](#712-ma-trận-phân-công-công-việc-tích-hợp-thực-tế-ví-dụ-tpos-tích-hợp-loyalty)
   - 7.13 [Các Nguyên Tắc Vàng (Golden Rules) Trong Kiến Trúc Company](#713-các-nguyên-tắc-vàng-golden-rules-trong-kiến-trúc-company)
8. [TỔNG HỢP DANH MỤC HỒ SƠ & TIÊU CHUẨN LƯU TRỮ TUÂN THỦ ISO/IEC 27001:2022](#8-tổng-hợp-danh-mục-hồ-sơ--tiêu-chuẩn-lưu-trữ-tuân-thủ-isoiec-270012022)
9. [QUY TRÌNH QUẢN LÝ SỰ CỐ HỆ THỐNG VÀ BẢO ĐẢM HOẠT ĐỘNG LIÊN TỤC (INCIDENT MANAGEMENT & DISASTER RECOVERY - ISO/IEC 27001:2022 A.5.24-A.5.29, A.8.13)](#9-quy-trình-quản-lý-sự-cố-hệ-thống-và-bảo-đảm-hoạt-động-liên-tục-incident-management--disaster-recovery---isoiec-270012022-a524-a529-a813)
   - 9.1 [Ma Trận Mức Độ Nghiêm Trọng (Severity Matrix P1-P4) & Cam Kết SLA](#91-ma-trận-mức-độ-nghiêm-trọng-severity-matrix-p1-p4--cam-kết-sla)
   - 9.2 [Kênh Cảnh Báo (Alerting Channels) & Quy Trình Trực On-Call 24/7](#92-kênh-cảnh-báo-alerting-channels--quy-trình-trực-on-call-247)
   - 9.3 [Quy Trình Phân Tích Nguyên Nhân Gốc Rễ (RCA) & Blameless Postmortem (48h)](#93-quy-trình-phân-tích-nguyên-nhân-gốc-rễ-rca--blameless-postmortem-48h)
   - 9.4 [Giám Sát Sau Release (Post-Release Monitoring) & Ngăn Ngừa Sự Cố Giới Hạn Tài Nguyên](#94-giám-sát-sau-release-post-release-monitoring--ngăn-ngừa-sự-cố-giới-hạn-tài-nguyên)
   - 9.5 [Kế Hoạch Khôi Phục Sau Thảm Họa (Disaster Recovery - DR) & BCP (ISO A.8.13, A.5.29)](#95-kế-hoạch-khôi-phục-sau-thảm-họa-disaster-recovery---dr--bcp-iso-a813-a529)
10. [QUY ĐỊNH BẢO MẬT HỆ THỐNG VÀ DEVSECOPS (ISO/IEC 27001:2022 A.8 & NGHỊ ĐỊNH PDPD 13/2023/NĐ-CP)](#10-quy-định-bảo-mật-hệ-thống-và-devsecops-isoiec-270012022-a8--nghị-định-pdpd-132023nđ-cp)
    - 10.1 [Ma Trận Phân Quyền Truy Cập (Access Control Matrix - ISO A.8.2, A.8.3, A.8.4)](#101-ma-trận-phân-quyền-truy-cập-access-control-matrix---iso-a82-a83-a84)
    - 10.2 [Quy Trình Quản Lý Bí Mật (Secrets Management & Vault Governance - ISO A.8.24)](#102-quy-trình-quản-lý-bí-mật-secrets-management--vault-governance---iso-a824)
    - 10.3 [Cổng Kiểm Soát Bảo Mật Tự Động Trong CI/CD (Security Gates - ISO A.8.25, A.8.28)](#103-cổng-kiểm-soát-bảo-mật-tự-động-trong-cicd-security-gates---iso-a825-a828)
    - 10.4 [Tuân Thủ Quy Định Bảo Vệ Dữ Liệu Cá Nhân Theo Nghị Định 13/2023/NĐ-CP (PDPD Vietnam)](#104-tuân-thủ-quy-định-bảo-vệ-dữ-liệu-cá-nhân-theo-nghị-định-132023nđ-cp-pdpd-vietnam)
11. [BỘ CHỈ SỐ DORA METRICS VÀ ĐO LƯỜNG NĂNG LỰC SẢN XUẤT (AGILE & ENGINEERING DASHBOARD)](#11-bộ-chỉ-số-dora-metrics-và-đo-lường-năng-lực-sản-xuất-agile--engineering-dashboard)
    - 11.1 [Bộ Chỉ Số DORA Metrics Bắt Buộc](#111-bộ-chỉ-số-dora-metrics-bắt-buộc)
    - 11.2 [Chỉ Số KPIs Chất Lượng Kỹ Thuật & Hiệu Suất Agile (Engineering Quality KPIs)](#112-chỉ-số-kpis-chất-lượng-kỹ-thuật--hiệu-suất-agile-engineering-quality-kpis)
    - 11.3 [Báo Cáo Tự Động Từ Harness Database (Sprint & Release Report Automation)](#113-báo-cáo-tự-động-từ-harness-database-sprint--release-report-automation)
12. [QUẢN TRỊ PHÁT TRIỂN PHẦN MỀM CÓ HỖ TRỢ TRÍ TUỆ NHÂN TẠO (AI-ASSISTED DEVELOPMENT GOVERNANCE & LLM POLICY)](#12-quản-trị-phát-triển-phần-mềm-có-hỗ-trợ-trí-tuệ-nhân-tạo-ai-assisted-development-governance--llm-policy)
    - 12.1 [Quy Định Quản Trị Mã Nguồn Do AI Tạo (Human-in-the-Loop & Tagging `[AI]`)](#121-quy-định-quản-trị-mã-nguồn-do-ai-tạo-human-in-the-loop--tagging-ai)
    - 12.2 [Bảo Mật Thông Tin & Quy Định Dữ Liệu Khi Sử Dụng LLM (Data Privacy & Secret Prohibition)](#122-bảo-mật-thông-tin--quy-định-dữ-liệu-khi-sử-dụng-llm-data-privacy--secret-prohibition)
    - 12.3 [Cổng Phê Duyệt Tự Động & Giới Hạn Quyền AI Agent (Approval Gates & Restricted Permissions)](#123-cổng-phê-duyệt-tự-động--giới-hạn-quyền-ai-agent-approval-gates--restricted-permissions)
    - 12.4 [Audit Logging & Observability Cho AI Agent (Harness Observability Audit Trail)](#124-audit-logging--observability-cho-ai-agent-harness-observability-audit-trail)

---

## 1. QUY TRÌNH 1: TIẾP NHẬN YÊU CẦU VÀ LẬP KẾ HOẠCH PHÁT TRIỂN SẢN PHẨM (SM-QT-01)

### 1.1 Mục Đích & Phạm Vi Áp Dụng
- **Mục đích**: Quy định trình tự và cách thức tiến hành tiếp nhận yêu cầu, đánh giá tính khả thi và lập kế hoạch phát triển sản phẩm nhằm đảm bảo tiếp nhận đầy đủ mong muốn của Stakeholders, xác định định hướng tương thích với hệ sinh thái sản phẩm, giảm thiểu rủi ro và tối ưu hóa sử dụng nguồn lực.
- **Phạm vi áp dụng**:
  - Các hoạt động tiếp nhận và lập kế hoạch thuộc Hệ thống quản lý an toàn thông tin (ISMS) của Công ty.
  - Áp dụng cho các hoạt động tiếp nhận định kỳ hoặc đột xuất theo yêu cầu ISO/IEC 27001:2022.
  - Áp dụng cho toàn bộ các dự án thuộc Phòng Phát triển Sản phẩm.

### 1.2 Trách Nhiệm Thực Hiện
- **Business Analyst (BA)**:
  - Tiếp nhận ý tưởng, yêu cầu từ Stakeholders.
  - Nghiên cứu, phân tích định hướng sản phẩm nhằm ngăn ngừa các ý tưởng không khả thi từ đầu.
  - Báo cáo kết quả nghiên cứu cho Stakeholders.
  - Phân tích người dùng, đề xuất giải pháp hiện thực hóa ý tưởng trên phần mềm.
  - Xác định danh sách tính năng (Feature set) cần thực hiện trong khoảng thời gian xác định.
  - Báo cáo và trình bày kế hoạch phác thảo với Stakeholders và các Trưởng bộ phận.
  - Hoàn chỉnh **Roadmap Final** sau khi thống nhất với các bên liên quan.
- **Trưởng các bộ phận trong phòng Phát triển Sản phẩm**:
  - Tiếp nhận nội dung, kế hoạch BA phân tích và soạn thảo.
  - Đề xuất, đóng góp ý kiến để hoàn thiện Roadmap.
- **Stakeholders - Ban Giám Đốc Công Ty**:
  - Đưa ra ý tưởng, yêu cầu muốn có trên sản phẩm phần mềm.
  - Tiếp nhận báo cáo trình bày và xác nhận định hướng phát triển sau nghiên cứu từ BA.
  - Tiếp nhận báo cáo giải pháp, tính năng và thời gian dự kiến hoàn thành.
  - Xác nhận phạm vi, thời gian thực hiện và Feature set cần thực hiện.

### 1.3 Thuật Ngữ & Định Nghĩa
| Thuật ngữ | Giải thích định nghĩa |
| :--- | :--- |
| **ISMS** | Hệ thống quản lý an toàn thông tin (Information Security Management System). |
| **Bộ phận** | Phòng, ban hoặc Trung tâm trực thuộc Công ty. |
| **ATTT** | An toàn thông tin. |
| **BGĐ / ĐĐLĐ** | Ban giám đốc / Đại diện lãnh đạo. |
| **BA** | Business Analyst - Chuyên viên phân tích nghiệp vụ và hệ thống. |
| **Stakeholders** | Các bên liên quan có ảnh hưởng hoặc bị ảnh hưởng bởi sản phẩm phần mềm (Giám đốc công ty, Giám đốc kỹ thuật, Khách hàng). |
| **Team** | Đội, nhóm thực hiện phát triển sản phẩm phần mềm. |
| **Feature set** | Danh sách tập hợp các tính năng của sản phẩm. |
| **Roadmap** | Lộ trình kế hoạch đạt được mục tiêu sản phẩm. |
| **Prototype** | Bản mẫu giao diện thể hiện ý tưởng trực quan. |

### 1.4 Sơ Đồ & Các Bước Thực Hiện Chi Tiết

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                        LUỒNG QUY TRÌNH TIẾP NHẬN & LẬP KẾ HOẠCH                        │
└────────────────────────────────────────────────────────────────────────────────────────┘

  [1. Truyền đạt ý tưởng] (Stakeholders / BA) 
           │
           ▼
  [2. Đánh giá & Nghiên cứu yêu cầu] (BA) 
           │
           ▼
  [3. Xác nhận định hướng] (Stakeholders + BA) 
           ├───► [Không đạt / Lệch ý tưởng] ──► Phân tích lại nguyên nhân
           └───► [Đạt / Tiềm năng phát triển]
                       │
                       ▼
  [4. Phân tích chi tiết & Lập kế hoạch sơ bộ] (BA)
                       │
                       ▼
  [5. Trao đổi kế hoạch sơ bộ với Stakeholders] (BA + Stakeholders)
                       ├───► [Kế hoạch không phù hợp] ──► Điều chỉnh lại từ bước 4
                       └───► [Kế hoạch phù hợp]
                                   │
                                   ▼
  [6. Trao đổi kế hoạch với các bên kỹ thuật] (BA + CTO, Tech Lead, UI/UX, SM)
                       ├───► [Kế hoạch thay đổi] ──► Điều chỉnh lại từ bước 4
                       └───► [Kế hoạch đáp ứng được]
                                   │
                                   ▼
  [7. Hoàn thiện & Lưu hồ sơ Roadmap Final] (BA)
```

- **Bước 5.1: Truyền đạt ý tưởng, yêu cầu**
  - Định kỳ 3–6 tháng (hoặc đột xuất), Stakeholders đưa ra ý tưởng/yêu cầu mới nằm trong tầm nhìn phát triển hệ sinh thái.
  - Nội dung yêu cầu tối thiểu gồm: (1) Vấn đề khách hàng gặp phải là gì? (2) Khách hàng mong muốn gì trên phần mềm? (3) Giá trị mang lại cho khách hàng khi giải quyết vấn đề?
  - BA tiếp nhận, tổng hợp và lập *Biên bản tiếp nhận ý tưởng, yêu cầu*.
- **Bước 5.2: Đánh giá & nghiên cứu yêu cầu đã tiếp nhận**
  - BA phân tích chi tiết: Market Potential (tiềm năng thị trường), Product Vision/Goals (tầm nhìn/mục tiêu), Target Customers (khách hàng mục tiêu), Value Proposition (giá trị đề xuất).
  - BA lập bản báo cáo kết quả nghiên cứu trình Stakeholders.
- **Bước 5.3: Xác nhận định hướng phát triển sau research**
  - BA và Stakeholders họp trao đổi kết quả phân tích:
    - *Trường hợp 1 (Có tiềm năng phát triển)*: Chuyển sang Bước 5.4.
    - *Trường hợp 2 (Không có tiềm năng)*: Đánh giá nguyên nhân. Nếu **Không khả thi** ➔ Dừng thực hiện. Nếu **Lệch ý tưởng** ➔ Stakeholders làm rõ lại để BA phân tích lại từ Bước 5.2.
- **Bước 5.4: Phân tích chi tiết - lập kế hoạch thực hiện sơ bộ**
  - BA lập kế hoạch sơ bộ gồm: Feature set, Mô tả & tiêu chí hoàn thành feature, Giải pháp đề xuất, Thứ tự ưu tiên, Thời gian thực hiện từng tính năng, Thời gian hoàn thành tổng thể, Prototype (nếu có).
- **Bước 5.5: Trao đổi kế hoạch sơ bộ với Stakeholders**
  - BA trao đổi với Stakeholders về danh sách tính năng, độ ưu tiên và thời gian hoàn thành.
  - *Nếu Phù hợp*: Chuyển sang Bước 5.6. *Nếu Không phù hợp*: Ghi nhận ý kiến và quay lại Bước 5.4 để điều chỉnh.
- **Bước 5.6: Trao đổi kế hoạch sơ bộ với các bên liên quan đến phát triển sản phẩm**
  - BA tổ chức họp với Tech Team gồm: CTO, Tech Lead, UI/UX, Scrum Master. BA gửi trước kế hoạch ít nhất **2 ngày**.
  - Đánh giá khả năng đáp ứng: Mục tiêu sản phẩm, thời gian, nguồn lực, giải pháp kỹ thuật (technical stack).
  - *Nếu Đáp ứng được*: Chuyển sang Bước 5.7. *Nếu Kế hoạch thay đổi*: Quay về Bước 5.4 để phân tích và điều chỉnh lại.
- **Bước 5.7: Hoàn thiện và lưu hồ sơ**
  - BA hoàn thiện bản kế hoạch Roadmap Final, chuyển hồ sơ lưu trữ và bàn giao sang quy trình thực hiện phát triển phần mềm.

### 1.5 Danh Mục Hồ Sơ Lưu Trữ (SM-QT-01)
| STT | Tên hồ sơ | Trách nhiệm lưu | Thời gian lưu trữ |
| :---: | :--- | :---: | :---: |
| 1 | Biên bản tiếp nhận ý tưởng, yêu cầu | BA | **1 năm** |
| 2 | Biên bản đánh giá định hướng ý tưởng, yêu cầu | BA | **1 năm** |
| 3 | Kế hoạch Roadmap Final | BA | **3 năm** |

---

## 2. QUY TRÌNH 2: THỰC HIỆN PHÁT TRIỂN PHẦN MỀM THEO SCRUM FRAMEWORK (SM-QT-02)

### 2.1 Mục Đích & Phạm Vi Áp Dụng
- **Mục đích**: Thiết lập quy trình phát triển phần mềm có cấu trúc các bước lặp đi lặp lại (Iterative & Incremental) nhằm nâng cao hiệu quả sản xuất, nhịp nhàng tương tác giữa các vai trò, ứng phó rủi ro kịp thời và đảm bảo chất lượng phần mềm theo cam kết.
- **Phạm vi áp dụng**: Tất cả các hoạt động phát triển phần mềm thuộc Phòng Phát triển Sản phẩm tuân thủ **Scrum Framework (Phiên bản tái bản 2020)** và tiêu chuẩn ISO/IEC 27001:2022.

### 2.2 Trách Nhiệm Của Các Vai Trò Trong Scrum
- **Business Analytics (BA as PO - Product Owner)**:
  - Tạo và truyền đạt nội dung chi tiết User Story minh bạch, dễ hiểu cho Dev Team.
  - Truyền đạt mục tiêu và kế hoạch phát triển sản phẩm.
  - Phối hợp với Dev Team sàng lọc, sắp xếp ưu tiên các User Story đảm bảo tiến độ.
  - Có quyền ủy quyền thực hiện nhưng chịu trách nhiệm cuối cùng về Product Backlog.
- **Development Team (Devs / UI-UX / Tester)**:
  - Tiếp nhận, đánh giá kế hoạch và mục tiêu sản phẩm.
  - Tham gia đầy đủ các sự kiện Scrum đúng quy định.
  - Lập kế hoạch và ước lượng nỗ lực cho mỗi Sprint.
  - Chuyển hóa mô tả User Story thành các phần tăng trưởng phần mềm (Increment) sử dụng được.
  - Tuân thủ nghiêm ngặt Định nghĩa hoàn thành (Definition of Done - DoD).
  - Thực hiện họp Daily Scrum hằng ngày đúng 15 phút.
- **Scrum Master (SM)**:
  - Hỗ trợ team tạo ra các Increment giá trị cao đáp ứng DoD.
  - Đảm bảo các sự kiện Scrum diễn ra đúng Timebox theo Scrum Guide.
  - Loại bỏ các cản trở (blockers) ảnh hưởng đến tiến độ Sprint Goal.
  - Thiết lập các báo cáo biên bản sau mỗi sự kiện họp Sprint.
  - Hỗ trợ BA theo dõi tiến độ, lập kế hoạch release và điều chỉnh khi có rủi ro.
- **Stakeholders**:
  - Tham gia họp Sprint Review cuối mỗi Sprint.
  - Đánh giá, góp ý trên các Increment thực tế được demo.

### 2.3 Thuật Ngữ & Định Nghĩa Cốt Lõi (Bao gồm DoR & Estimation)
| Thuật ngữ | Giải thích định nghĩa |
| :--- | :--- |
| **DoR (Definition of Ready)** | Định nghĩa sẵn sàng - Tiêu chuẩn bắt buộc cho một User Story trước khi được phép kéo vào Sprint Planning. |
| **DoD (Definition of Done)** | Định nghĩa hoàn thành - Tiêu chuẩn chất lượng bắt buộc bắt buộc phải đạt được trước khi một hạng mục công việc được coi là hoàn tất. |
| **Story Points** | Đơn vị đo lường tương đối nỗ lực, độ phức tạp và rủi ro của User Story (tuân thủ dãy Fibonacci). |
| **Timebox** | Khoảng thời gian giới hạn tối đa cho phép ở mỗi sự kiện họp. |
| **Product Backlog** | Danh sách sắp xếp theo ưu tiên chứa toàn bộ các hạng mục cần thực hiện cho sản phẩm. |
| **Sprint Backlog** | Danh sách các hạng mục công việc được chọn để thực hiện trong Sprint hiện tại kèm kế hoạch triển khai. |
| **Increment** | Phần tăng trưởng sản phẩm hoàn chỉnh, kiểm thử đạt DoD mà người dùng có thể thao tác sử dụng được ngay. |
| **Value increase opportunities** | Cơ hội gia tăng giá trị sản phẩm qua việc thay đổi thứ tự ưu tiên hoặc cải tiến User Story. |

#### 2.3.1 Definition of Ready (DoR) - Tiêu chuẩn sẵn sàng cho User Story
Một User Story **CHỈ ĐƯỢC PHÉP KÉO VÀO SPRINT** khi đáp ứng đủ 6 tiêu chí DoR bắt buộc sau:
1. **Clear Business Value & Context**: Mô tả rõ ràng vai trò người dùng (As a), hành động (I want), giá trị mang lại (So that).
2. **Detailed Acceptance Criteria (AC)**: Có danh sách tiêu chí nghiệm thu rõ ràng, không mập mờ (viết theo dạng Given-When-Then hoặc Checklist chi tiết).
3. **UI/UX Design Sign-off**: Đã có đầy đủ Mockup/Prototype giao diện được BA và UI/UX duyệt (đối với User Story có giao diện).
4. **Technical & Dependency Identification**: Đã xác định rõ các API contract, data fields, schema DB và không còn dependency bị nghẽn bởi team khác.
5. **Story Points Estimated**: Đã được toàn bộ Dev Team đồng thuận ước lượng điểm nỗ lực theo Fibonacci trong buổi Grooming/Refinement.
6. **Testability**: Tester đã rà soát và xác nhận User Story có thể viết được Testcase kiểm thử độc lập.

#### 2.3.2 Kỹ thuật Ước lượng Effort theo Dãy Fibonacci & Planning Poker
- **Dãy Fibonacci chuẩn áp dụng**: `1, 2, 3, 5, 8, 13` Story Points.
- **Nguyên tắc phân rã**: Nếu một User Story được ước lượng **> 8 Story Points (ví dụ 13 hoặc lớn hơn)**, User Story đó **BẮT BUỘC phải được phân rã thành các User Story nhỏ hơn** (≤ 8 Story Points) trước khi đưa vào Sprint.
- **Phương pháp Planning Poker**:
  - BA trình bày User Story và giải đáp thắc mắc.
  - Mỗi thành viên Dev Team chọn ẩn 1 con số trong dãy Fibonacci thể hiện độ phức tạp/nỗ lực.
  - Đồng thời mở bài: Nếu có sự chênh lệch lớn (ví dụ 2 điểm vs 8 điểm), hai thành viên chọn điểm thấp nhất và cao nhất cùng giải thích lý do để team thảo luận chốt điểm đồng thuận.

### 2.4 Sơ Đồ & Chi Tiết Các Sự Kiện Sprint (Sprint Events)

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                        CHUYÊN CHU TRÌNH VÒNG LẶP SPRINT (SCRUM LOOP)                    │
└────────────────────────────────────────────────────────────────────────────────────────┘

  [Product Backlog]
         │
         ▼
  [1. Grooming Meeting] ──► Phân rã & Làm rõ User Stories
         │
         ▼
  [2. Sprint Planning] ──► Chốt Sprint Goal & Sprint Backlog
         │
         ▼
  ┌────────────────────────────────────────────────────────┐
  │ 🔄 CHU KỲ PHÁT TRIỂN SPRINT (1 - 2 Tuần)              │
  │                                                        │
  │   [3. Daily Scrum (15 phút/ngày)] ──► Cập nhật tiến độ  │
  │   [Dev & Test Implementation]     ──► Kiểm thử đạt DoD │
  └───────────────────────────┬────────────────────────────┘
                              │
                              ▼
  [4. Sprint Review & Demo] ──► Demo Increment cho Stakeholders
                              │
                              ▼
  [5. Sprint Retrospective] ──► Cải tiến quy trình cho Sprint sau
                              │
                              ▼
  [6. Hoàn thiện & Chuẩn bị Delivery] ──► Đưa vào bản Kế hoạch Release
```

- **5.1 Grooming (Họp làm rõ Backlog)**
  - *Thành phần*: BA (as PO), Dev Team, Scrum Master, Stakeholders (nếu cần).
  - *Mục đích*: Truyền đạt, giải thích chi tiết User Story và cam kết kế hoạch.
  - *Nội dung*: BA trình bày danh sách User Story (kèm Activity Diagram, Business rules, UI mockup, fields). Dev Team đặt câu hỏi, góp ý và tiến hành ước lượng sơ bộ.
  - *Input*: Kế hoạch sản phẩm, danh sách User Stories, UI/Mockups.
  - *Output*: Danh sách User Story cam kết sẵn sàng triển khai, bản phân rã công việc sơ bộ cho các Sprint.
- **5.2 Sprint Planning (Lập kế hoạch Sprint)**
  - *Thành phần*: BA (as PO), Dev Team, Scrum Master, Stakeholders (nếu được mời).
  - *Nội dung*: BA trình bày mục tiêu Sprint và các Value increase opportunities. Scrum Master tính toán Velocity và kiểm soát Timebox. Dev Team đặt vấn đề giải pháp kỹ thuật, thảo luận task breakdown và estimate Story Points.
  - *Input*: Product Backlog, Team Capacity, DoD, Past Performance.
  - *Output*: **Sprint Goal**, Danh sách Sprint Backlog và Kế hoạch thực hiện chi tiết.
- **5.3 Daily Scrum (Họp hằng ngày)**
  - *Thành phần*: Developers / QA (Dev Team).
  - *Timebox*: **Tối đa 15 phút** mỗi ngày tại một khung giờ thống nhất.
  - *Mô tả*: Mỗi cá nhân trả lời 3 câu hỏi hướng tới Sprint Goal: (1) Đã làm gì ngày qua? (2) Sẽ làm gì hôm nay? (3) Gặp khó khăn/vướng mắc gì?
  - *Output*: Sprint Backlog được cập nhật realtime, các cản trở được ghi nhận để SM xử lý.
- **5.4 Sprint Review (Họp nghiệm thu Sprint)**
  - *Thành phần*: Dev Team, BA (as PO), Scrum Master, Stakeholders.
  - *Nội dung*: Dev Team **demo trực tiếp các Increment** đã hoàn thành đạt DoD. BA nghiệm thu các hạng mục đạt/chưa đạt. Stakeholders đặt câu hỏi và phản hồi ý kiến.
  - *Input*: Increment hoàn thành DoD, Kịch bản demo.
  - *Output*: Product Backlog được điều chỉnh, Danh sách phản hồi của Stakeholders, Kế hoạch Release cập nhật.
- **5.5 Sprint Retrospective (Họp cải tiến Sprint)**
  - *Thành phần*: Scrum Master (Host), BA (as PO), Dev Team.
  - *Mục đích*: Đánh giá hiệu suất làm việc của Sprint hiện tại và đưa ra hành động cải tiến cho Sprint sau.
  - *Nội dung*: Đóng góp ý kiến theo mẫu (Good - Bad - Ideas), thảo luận giải quyết các mâu thuẫn và xung đột quy trình.
  - *Output*: Bảng kế hoạch hành động cải tiến (Action items cho Sprint kế tiếp).
- **5.6 Hoàn thiện và chuẩn bị Delivery**
  - BA tổng hợp toàn bộ Increment đã hoàn thành trong các Sprint. SM phối hợp với BA đánh giá tiến độ tổng thể và chia giai đoạn phát hành (Release phases) theo tình hình thị trường.

### 2.5 Danh Mục Hồ Sơ Lưu Trữ (SM-QT-02)
| STT | Tên hồ sơ | Trách nhiệm lưu | Thời gian lưu trữ |
| :---: | :--- | :---: | :---: |
| 1 | Report Sprint planning meeting | Scrum Master | **1 năm** |
| 2 | Report Sprint Review meeting | Scrum Master | **1 năm** |
| 3 | Plan Release | BA / SM | **3 năm** |
| 4 | Product backlog | BA | **2 năm** |
| 5 | Sprint backlog | Dev Team / SM | **1 tháng** |

---

## 3. QUY TRÌNH 3: PHÁT HÀNH SẢN PHẨM PHẦN MỀM - RELEASE (SM-QT-03)

### 3.1 Mục Đích & Phạm Vi Áp Dụng
- **Mục đích**: Đảm bảo toàn bộ các bộ phận nắm rõ nội dung trước khi triển khai chính thức lên môi trường Production, xác định tính khả thi, chuẩn hóa các bước phát hành nhằm giảm thiểu rủi ro gián đoạn dịch vụ và đảm bảo các tính năng đạt tiêu chuẩn kiểm định chất lượng.
- **Phạm vi áp dụng**: Tất cả các đợt phát hành định kỳ hoặc đột xuất (Hotfix) áp dụng cho toàn bộ dự án thuộc Phòng Phát triển Sản phẩm, tuân thủ tiêu chuẩn an toàn thông tin ISO/IEC 27001:2022 và Scrum Framework.

### 3.2 Trách Nhiệm Thực Hiện Phát Hành
- **Stakeholders**: Truyền đạt mục tiêu, phối hợp ưu tiên lộ trình triển khai và tiếp nhận kế hoạch phát hành.
- **Business Analyst (BA)**: Cầu nối thông tin, lập Kế hoạch Release trình duyệt, theo dõi tiến độ phát hành.
- **Nhóm dự án (Dev Team / DevOps)**: Đảm bảo thực hiện công việc đúng kế hoạch, liên hệ phòng Hạ tầng cấu hình hệ thống trên môi trường Production.
- **Tester (QA)**: Xây dựng Test Plan/Test Cases, kiểm thử nghiệm thu trên môi trường Staging, lưu vết backlog tồn đọng, lập tài liệu hướng dẫn và bàn giao tính năng.
- **Scrum Master (SM)**: Tháo gỡ cản trở phát hành, hỗ trợ BA theo dõi tiến độ và lập kế hoạch release.

### 3.3 Thuật Ngữ & Định Nghĩa
| Thuật ngữ | Giải thích định nghĩa |
| :--- | :--- |
| **Release** | Phát hành - Hoạt động đóng gói và triển khai phần mềm lên môi trường cho người dùng sử dụng. |
| **Môi trường Staging** | Môi trường giả lập trung gian giống môi trường Production tới 99%, dùng để kiểm thử cuối cùng. |
| **Môi trường Production** | Môi trường vận hành chính thức nơi người dùng cuối trực tiếp thao tác khai thác phần mềm. |
| **Hotfix** | Bản vá lỗi khẩn cấp được triển khai ngay lập tức để xử lý sự cố nghiêm trọng trên Production. |
| **Test Release Report** | Báo cáo tổng kết kết quả kiểm thử phiên bản phát hành. |

### 3.4 Sơ Đồ & Luồng Triển Khai Phát Hành Chi Tiết

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                          LUỒNG QUY TRÌNH PHÁT HÀNH SẢN PHẨM                            │
└────────────────────────────────────────────────────────────────────────────────────────┘

  [Thông qua nội dung Release] (User Stories đạt trạng thái CLOSED)
               │
               ▼
  [1. Lập Kế hoạch Release] (BA) ──► Trình Stakeholders phê duyệt
               │
               ▼
  [2. Lập Kế hoạch Test] (Tester) ──► Trình PO / Trưởng dự án phê duyệt
               │
               ▼
  [3. Triển khai bản Release lên môi trường STAGING] (Dev Team / DevOps)
               │
               ▼
  [4. Kiểm thử trên môi trường STAGING] (Tester)
               ├───► [KẾT QUẢ KHÔNG ĐẠT] ──► Report lỗi & Lưu Backlog tồn đọng
               │                              (Cập nhật lùi ngày Kế hoạch Release)
               └───► [KẾT QUẢ ĐẠT]
                           │
                           ├────────────────────────────────────────┐
                           ▼                                        ▼
             [5. Lập Tài liệu mô tả & Bàn giao]         [6. Deploy bản Release lên PRODUCTION]
             (Tester bàn giao cho các bộ phận)          (Devops / Dev Team triển khai)
                                                                    │
                                                                    ▼
                                                        [7. Theo dõi & Đánh giá kết quả]
                                                        (Thu thập phản hồi trong 2 ngày)
                                                                    │
                                                                    ├──► [Phát hiện lỗi nghiêm trọng] ➔ Hotfix
                                                                    └──► [Hệ thống ổn định] ➔ Kết thúc Release
```

- **5.1.1 Thông qua nội dung Release**: **CHỈ NHỮNG USER STORY ĐÃ ĐẠT TRẠNG THÁI CLOSED** (đã qua nghiệm thu Sprint Review) mới đủ điều kiện được đưa vào Kế hoạch Release.
- **5.1.2 Lập và phê duyệt kế hoạch Release**:
  - BA lập bản Kế hoạch Release chi tiết.
  - **QUY ĐỊNH BẮT BUỘC VỀ THỜI GIAN**: Kế hoạch Release cho môi trường Production **CHỈ ĐƯỢC PHÉP LẬP VÀO CÁC NGÀY CHỦ NHẬT, THỨ HAI, THỨ BA VÀ THỨ TƯ trong tuần**. Việc này nhằm đảm bảo hệ thống có đủ nhân sự hỗ trợ theo dõi sau phát hành, tuyệt đối hạn chế phát hành cận cuối tuần.
  - Kế hoạch Release phải có chữ ký phê duyệt chính thức từ Stakeholders / Giám đốc.
- **5.1.3 Lập và phê duyệt kế hoạch Test**: Tester lập Kế hoạch Test chi tiết dựa trên scope của Kế hoạch Release. Được duyệt bởi PO hoặc Trưởng dự án.
- **5.2.1 & 5.2.2 Triển khai và Kiểm thử trên Staging**:
  - **BẮT BUỘC TRÊN STAGING**: Mọi phiên bản trước khi lên Production đều phải được deploy và pass test trên môi trường Staging.
- **5.2.3 Báo cáo kết quả Staging & Xử lý sự cố**:
  - Nếu kết quả kiểm thử không đạt: Tester lưu lại backlog tồn đọng. Kế hoạch Release chỉ cho phép lùi thời gian phát hành sang ngày dự kiến mới.
- **5.3.1 & 5.3.2 Lập tài liệu mô tả và Bàn giao**:
  - Tester (hoặc người được ủy quyền) lập tài liệu mô tả toàn bộ tính năng release, tổ chức bàn giao hướng dẫn cho các bộ phận Vận hành, CSKH, Marketing.
- **5.4.1 Triển khai Production**: Dev Team / DevOps thực hiện triển khai bản release lên môi trường Production.
- **5.4.2 & 5.4.3 Nhận phản hồi & Đánh giá kết quả**:
  - Toàn bộ nhóm dự án theo dõi sát sao sau phát hành. Nếu phát sinh sự cố nghiêm trọng ➔ Kích hoạt quy trình **Hotfix** ngay lập tức.
  - Đánh giá kết quả phát hành được thực hiện trong vòng **2 ngày làm việc** sau release dựa trên: (1) Độ ổn định hệ thống; (2) Độ hoàn thiện tài liệu bàn giao.

### 3.5 Quy Định Khung Thời Gian & Phê Duyệt Release
- **Ngày lập kế hoạch & Release Production**: Chủ nhật, Thứ 2, Thứ 3, Thứ 4.
- **Phê duyệt Kế hoạch Release**: Phải có chữ ký Stakeholders.
- **Phê duyệt Kế hoạch Test**: Phải có chữ ký PO / Trưởng dự án.

### 3.6 Danh Mục Hồ Sơ Lưu Trữ (SM-QT-03)
| STT | Tên hồ sơ | Trách nhiệm lưu | Thời gian lưu trữ |
| :---: | :--- | :---: | :---: |
| 1 | Kế hoạch release | BA / PO | **3 năm** |
| 2 | Kế hoạch Test | Tester | **3 năm** |
| 3 | Test Release Report | Tester | **3 năm** |
| 4 | Product backlog | BA | **2 năm** |
| 5 | Tài liệu mô tả tính năng, chức năng | Tester | **Lưu đến khi tài liệu không còn chính xác với thực tế sản phẩm** |

---

## 4. QUY TRÌNH 4: XỬ LÝ YÊU CẦU BỔ SUNG TRONG MÔ HÌNH MICROSERVICES (SM-QT-04)

### 4.1 Mục Đích & Nguyên Tắc Phối Hợp Provider - Tenant
- **Mục đích**: Thiết lập quy trình chuẩn xử lý các yêu cầu bổ sung tính năng giữa các đội nhóm phát triển trong kiến trúc Microservices, phân định rõ ràng giữa **Provider** (Team sở hữu service) và **Tenant** (Team có nhu cầu sử dụng/bổ sung service).
- **Nguyên tắc cốt lõi**:
  - Đảm bảo tính minh bạch requirement từ sớm qua mẫu **ORF (Output Request Form)**.
  - Áp dụng SLA tiếp nhận nghiêm ngặt.
  - Bắt buộc xác nhận chất lượng qua 2 vòng kiểm thử: Môi trường Dev (có xác nhận của Tenant) và Môi trường Staging trước khi lên Production.

### 4.2 Trách Nhiệm Các Bên & SLA Tiếp Nhận (24h)
- **Provider (Team sở hữu Service)**:
  - **SLA**: Phản hồi tiếp nhận ORF từ Tenant trong vòng **24 giờ làm việc**.
  - Phân tích khả năng thực hiện, tác động hệ thống.
  - Chủ trì họp 2-Team meeting để chốt requirement & timeline.
  - Tiến hành code và deploy môi trường Dev/Staging.
- **Tenant (Team sử dụng Service)**:
  - Mô tả rõ ràng nhu cầu qua biểu mẫu ORF.
  - Tham gia 2-Team meeting.
  - Cung cấp tài liệu tích hợp (API Docs, Schema, ERD) khi cần implement chung.
  - Kiểm tra và xác nhận (sign-off) tính năng trên môi trường Dev do Provider cung cấp.
- **CTO / CPO**:
  - Trực tiếp giải quyết (Escalate) khi có tranh chấp phạm vi hoặc bất đồng hướng triển khai giữa Provider và Tenant. Quyết định Provider làm hay Tenant tự xây dựng microservice riêng.

### 4.3 Thuật Ngữ & Định Nghĩa
| Thuật ngữ | Giải thích định nghĩa |
| :--- | :--- |
| **ORF** | Output Request Form - Biểu mẫu mô tả yêu cầu bổ sung tính năng do Tenant gửi Provider. |
| **Provider** | Đội nhóm sở hữu và quản lý microservice được yêu cầu bổ sung. |
| **Tenant** | Đội nhóm có nhu cầu tích hợp hoặc bổ sung tính năng từ service của Provider. |
| **SLA (24h)** | Service Level Agreement - Cam kết phản hồi xác nhận ORF trong 24 giờ làm việc. |
| **Escalate** | Leo thang xử lý - Chuyển vấn đề lên cấp CTO/CPO quyết định khi 2 team không đạt thống nhất. |

### 4.4 Biểu Mẫu Output Request Form (ORF)
Mẫu ORF tiêu chuẩn gồm các mục bắt buộc:
1. **Bối cảnh**: Tính năng gì? Tại sao cần bổ sung?
2. **Yêu cầu cụ thể**: Cần thêm API mới, thêm field hay thay đổi logic xử lý hiện tại?
3. **Input / Output mong muốn**: Cấu trúc dữ liệu đầu vào và đầu ra.
4. **Deadline**: Thời gian Tenant cần tính năng vận hành.
5. **Trường hợp đặc biệt**: Các rủi ro hoặc edge-cases cần xử lý.
6. **Yêu cầu Release gấp**: Có cần phát hành ngay lập tức hay theo đợt?

### 4.5 Sơ Đồ & Luồng Xử Lý Cross-Team Chi Tiết

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                   LUỒNG XỬ LÝ YÊU CẦU BỔ SUNG MICROSERVICES (CROSS-TEAM)               │
└────────────────────────────────────────────────────────────────────────────────────────┘

  [Tenant gửi biểu mẫu ORF]
           │
           ▼
  [Provider tiếp nhận ORF] (SLA 24 giờ làm việc)
           │
           ▼
  [Kiểm tra Requirement Clear?]
           ├───► [NO - Chưa rõ ràng] ──► Họp 2-Team Meeting (Chốt Scope & Timeline)
           └───► [YES - Đã rõ ràng]
                       │
                       ▼
  [Phân tích khả năng thực hiện] (BA Provider)
           │
           ├───► [Đã có sẵn] ──► Cung cấp hướng dẫn tích hợp cho Tenant ➔ KẾT THÚC
           ├───► [Không thể làm] ──► Escalate CTO/CPO ──► Quyết định Provider làm / Tenant tự tạo service
           └───► [Có thể thực hiện]
                       │
                       ▼
  [Xác định Mức độ Ưu tiên] (BA Provider)
           ├───► [Ưu tiên Cao] ──► Chen vào Sprint hiện tại của Provider
           └───► [Ưu tiên Thấp] ──► Đưa vào Product Backlog cho Sprint sau
                       │
                       ▼
  [Dev Provider Code & Deploy Môi trường DEV]
           │
           ▼
  [Tester Provider Kiểm thử môi trường DEV] ➔ [Đạt]
           │
           ▼
  [BA / Dev Tenant nghiệm thu xác nhận trên Môi trường DEV] ➔ [Đạt]
           │
           ▼
  [Xác định Yêu cầu Release]
           ├───► [Release theo kế hoạch] ──► Đưa vào Plan Release định kỳ
           └───► [Release Ngay]
                       │
                       ▼
  [Provider deploy & Tester kiểm thử môi trường STAGING] (BẮT BUỘC)
                       │
                       ▼
  [Provider Release lên PRODUCTION]
                       │
                       ▼
  [Thông báo BA Tenant + Bàn giao tài liệu Technical Final]
```

### 4.6 Danh Mục Hồ Sơ Lưu Trữ (SM-QT-04)
| STT | Tên hồ sơ | Trách nhiệm lưu | Thời gian lưu trữ |
| :---: | :--- | :---: | :---: |
| 1 | ORF (Output Request Form) | Tenant / Provider BA | **3 năm** |
| 2 | Biên bản 2-team meeting | Provider BA | **3 năm** |
| 3 | Kế hoạch Release liên quan cross-team | Provider BA | **3 năm** |
| 4 | Kế hoạch Test | Tester | **3 năm** |
| 5 | Test Release Report | Tester | **3 năm** |
| 6 | Output Document (API Docs, Schema, ERD) | Tenant Dev | **3 năm** |
| 7 | Tài liệu mô tả tính năng, chức năng | Tester | **Lưu đến khi tài liệu không còn chính xác với thực tế** |

---

## 5. QUY ĐỊNH 5: QUẢN LÝ BACKLOG VÀ WORK ITEMS TRÊN TFS / JIRA (SM-QĐi-01)

### 5.1 Mục Đích & Phạm Vi Áp Dụng
- **Mục đích**: Thống nhất quy trình thao tác, cách phân tầng và chuẩn hóa cú pháp viết các loại Work Item trên phần mềm quản lý (TFS / Azure DevOps / Jira), đảm bảo tính chuyên nghiệp, thống nhất và dễ tra cứu.
- **Phạm vi áp dụng**: Tất cả nhân viên (BA, Dev, QA, SM) thao tác trên công cụ quản lý backlog thuộc Phòng Phát triển Sản phẩm.

### 5.2 Cấu Trúc Phân Tầng Work Items (Portfolio Backlog ➔ Issue Tracking)

```text
                               ┌───────────────────────────┐
                               │        PORTFOLIO          │
                               │          EPIC             │
                               └─────────────┬─────────────┘
                                             │
                               ┌─────────────▼─────────────┐
                               │          FEATURE          │
                               └─────────────┬─────────────┘
                                             │
                               ┌─────────────▼─────────────┐
                               │      PRODUCT BACKLOG      │
                               │        USER STORY         │
                               └──────┬──────┬──────┬──────┘
                                      │      │      │
           ┌──────────────────────────┘      │      └──────────────────────────┐
           ▼                                 ▼                                 ▼
┌────────────────────┐            ┌────────────────────┐            ┌────────────────────┐
│        TASK        │            │      TESTCASE      │            │        BUG         │
│ (Chi tiết kỹ thuật)│            │ (Kịch bản kiểm thử)│            │   (Lỗi hệ thống)   │
└────────────────────┘            └────────────────────┘            └────────────────────┘
```

- **EPIC**: Mục tiêu lớn cấp chiến lược (Business Value lớn), kéo dài qua nhiều Sprint, phân rã thành các Feature.
- **FEATURE**: Tập hợp các khả năng (Capabilities) thuộc Epic, phân rã thành các User Story.
- **USER STORY**: Yêu cầu chức năng cụ thể từ góc nhìn người dùng cuối do BA viết, phân rã thành các Task.
- **TASK**: Đơn vị công việc kỹ thuật nhỏ nhất giao cho Dev/Tester thực hiện.
- **BUG**: Bản ghi lưu trữ lỗi phát sinh trong quá trình kiểm thử hoặc vận hành.
- **TESTCASE**: Kịch bản kiểm thử dùng để xác minh tính năng.
- **ISSUE**: Sự cố bất thường ngoài kế hoạch do BA/Tech Lead/SM ghi nhận.

### 5.3 Thành Phần Chi Tiết & Quy Định Các Trường Dữ Liệu
| Trường (Field) | Mô tả & Quy chuẩn nhập dữ liệu |
| :--- | :--- |
| **Title** | Tiêu đề ngắn gọn tuân thủ đúng định dạng cú pháp chuẩn cho từng loại Work Item. |
| **Description** | Mô tả chi tiết bối cảnh, nội dung công việc hoặc yêu cầu. |
| **Acceptance Criteria** | Tiêu chí chấp nhận (Định nghĩa hoàn thành của hạng mục). |
| **Assigned To** | Người chịu trách nhiệm chính thực hiện. |
| **State** | Trạng thái vòng đời hiện tại của Work Item. |
| **Area Path** | Nhóm phân vùng sản phẩm / module tương ứng. |
| **Iteration Path** | Sprint / Chu kỳ áp dụng thực hiện. |
| **Tags** | Gắn nhãn phân loại (UI, BE, FE, API, Performance, Hotfix,...). |
| **Effort / Story Point** | Điểm số ước lượng nỗ lực công sức (Story Points cho US; giờ cho Task). |

### 5.4 Luồng Trạng Thái (State Machine) Của Từng Work Item

#### A. Luồng trạng thái User Story & Feedback Bug:
```text
[New] ──► [Accepted] ──► [Active] ──► [Resolved] ──► [Closed]
  │                           │
  └── (Hủy bỏ) ───────────────┴──► [Removed]
```
- **New**: Mới khởi tạo, chờ BA phân tích.
- **Accepted**: BA/UI-UX đã hoàn thiện tài liệu, sẵn sàng cho Sprint Planning.
- **Active**: Đang được triển khai trong Sprint.
- **Resolved**: Dev đã code xong, deploy & pass test trên Dev environment.
- **Closed**: Đã qua nghiệm thu Sprint Review, hoàn tất phát hành.

#### B. Luồng trạng thái Task:
```text
[New] ──► [Active] ──► [Resolved] ──► [Closed]
  │           │
  │           └──► [Blocked] (Vướng mắc tạm dừng)
  └──► [Removed]
```

#### C. Luồng trạng thái Bug:
```text
[New] ──► [Active] (Dev kéo sửa) ──► [Resolved] (Dev fix xong, chờ QA test) ──► [Closed] (QA xác nhận đã fix)
  │
  └── (Lỗi không đúng/trùng) ──► [Removed]
```

### 5.5 Quy Ước Chuẩn Viết Nội Dung Cho Epic, Feature, User Story, Task, Bug, Testcase

#### 5.5.1 Quy ước viết EPIC
- **Cú pháp Title**: `[Giá trị nghiệp vụ] – [Tác động chính đến người dùng / doanh nghiệp]`
  - *Ví dụ*: `Tăng hiệu suất bán hàng – Hỗ trợ nhân viên chăm sóc khách hàng hiệu quả hơn`
  - *Ví dụ*: `Tối ưu quy trình onboarding – Giảm thời gian tạo tài khoản xuống dưới 5 phút`
- **Nội dung mô tả (Description)**: Nhập mô tả tổng quan về mục tiêu Epic.
- **Business Value**:
  - *Ví dụ*: Tăng tỷ lệ giữ chân người dùng (Retention Rate), Cải thiện điểm CSAT từ 3.8 ➔ 4.5+.
- **Acceptance Criteria (Cấp Epic)**:
  - *Ví dụ*: Giao diện hiển thị tốt trên iPhone & Android; Thời gian tải trang < 3 giây.

#### 5.5.2 Quy ước viết FEATURE
- **Cú pháp Title**: `[Khả năng cụ thể] – [Module/Tính năng chính]`
  - *Ví dụ*: `Tìm kiếm khách hàng nâng cao – Module CRM`
  - *Ví dụ*: `Quản lý phân quyền – Giao diện quản trị`
  - *Ví dụ*: `Xuất báo cáo doanh thu – Dashboard`
- **Mô tả (Description)**: Mô tả tổng quan khả năng của Feature.
- **Acceptance Criteria**: Các tiêu chí cần đạt để hoàn thành Feature.

#### 5.5.3 Quy ước viết USER STORY
- **Cú pháp Title**: `[Module/Feature] – [Vai trò] có thể [hành động/mục tiêu]`
  - *Ví dụ*: `[Dashboard] – Quản lý có thể xem báo cáo doanh thu theo tháng`
  - *Ví dụ*: `[Checkout] – Khách hàng có thể thanh toán qua quét mã QR VNPay`
- **Mô tả (Description)**: Bắt buộc viết theo chuẩn format:
  > **As a** [Vai trò người dùng],  
  > **I want** [Hành động / Mong muốn],  
  > **So that** [Giá trị / Mục đích đạt được].
- **Acceptance Criteria (Bắt buộc)**: Liệt kê chi tiết các điều kiện nghiệm thu.
- **Story Points**: Số điểm nỗ lực được Dev Team đồng thuận.

#### 5.5.4 Quy ước viết TASK
- **Cú pháp Title**: `[Module hoặc tính năng] - [Động từ + hành động cụ thể]`
  - *Ví dụ*: `Login UI - Tạo giao diện đăng nhập`
  - *Ví dụ*: `Auth API - Viết API xác thực JWT`
- **Mô tả**: Ghi chú kỹ thuật triển khai.
- **Remaining Work**: Số giờ ước lượng hoàn thành.

#### 5.5.5 Quy ước viết TESTCASE
- **Cú pháp Title**: `[Module]: [Result expect] when [how to...]`
  - *Ví dụ*: `[Auth]: Hiển thị thông báo lỗi khi nhập sai mật khẩu 3 lần`
- **Steps**: Liệt kê từng bước thao tác kiểm thử.
- **Expected Result**: Kết quả kỳ vọng sau mỗi bước.

#### 5.5.6 Quy ước viết BUG
- **Cú pháp Title**: `[Module]: [Error message] when [why]`
  - *Ví dụ*: `[Payment]: Lỗi 500 Internal Server khi ấn thanh toán với giỏ hàng > 100 sản phẩm`
- **Repro Steps**: Các bước từng bước để tái hiện lỗi.
- **Actual Result**: Trạng thái lỗi thực tế đang diễn ra.
- **Expected Result**: Trạng thái đúng lẽ ra phải hoạt động.
- **Priority / Severity**: Mức độ nghiêm trọng của lỗi.
- **Tags**: Gắn nhãn phân loại (FE/BE/Performance/Environment).

### 5.6 Danh Mục Hồ Sơ Lưu Trữ (SM-QĐi-01)
| STT | Tên hồ sơ | Trách nhiệm lưu | Thời gian lưu trữ |
| :---: | :--- | :---: | :---: |
| 1 | Product backlog (Danh sách Epic, Feature, User Story) | BA | **1 năm** |
| 2 | Danh sách Task, Bug | Developers | **1 năm** |
| 3 | Testcase | Tester | **1 năm** |

---

## 6. QUY ĐỊNH 6: QUẢN LÝ WORK ITEM TRÊN TFS (SM-QĐ-002 - HIỆU LỰC 26/06/2026)

### 6.1 Mục Đích & Phạm Vi Áp Dụng
- **Mã tài liệu**: `SM-QĐ-002` | **Phiên bản**: `1.0` | **Ngày ban hành**: `26/06/2026`
- **Mục đích**:
  - Thống nhất cách thức quản lý, theo dõi và cập nhật trạng thái các work item trên hệ thống TFS (Team Foundation Server) trong toàn bộ đội nhóm phát triển phần mềm.
  - Đảm bảo minh bạch, truy xuất được quá trình phát triển, kiểm thử và phát hành sản phẩm.
  - Phân định rõ trách nhiệm của từng vai trò (CPO, BA, Dev, Tester, SM/PM) trong việc vận hành các work item.
  - Nâng cao chất lượng sản phẩm thông qua quy trình kiểm soát chặt chẽ ở từng giai đoạn.
- **Phạm vi áp dụng**:
  - *Đối tượng*: Toàn bộ thành viên đội nhóm phát triển phần mềm bao gồm CPO, BA (Business Analyst), Developer, Tester, Scrum Master (SM) / Project Manager (PM).
  - *Hệ thống*: Tất cả các work item được tạo và quản lý trên TFS của tổ chức.
  - *Môi trường*: Toàn bộ vòng đời phát triển từ Local ➔ Development (Dev/Test) ➔ Staging/UAT ➔ Production.

### 6.2 Trách Nhiệm Theo Vai Trò (CPO, BA, Dev, Tester, SM/PM)

| Vai trò | Trách nhiệm thực hiện quy định SM-QĐ-002 |
| :--- | :--- |
| **CPO (Chief Product Officer)** | Tạo và quản lý Epic; Closed User Story và Technical Story sau Sprint Review; Chuyển Closed cho Epic sau khi release Production. |
| **BA (Business Analyst)** | Tạo User Story; Chuyển trạng thái User Story từ `New` ➔ `Accepted` khi đã đầy đủ tài liệu kỹ thuật và thiết kế UI. |
| **Developer** | Tạo và thực hiện Task; Chuyển trạng thái User Story, Technical Story, Task trong quá trình làm việc; Fix bug. |
| **Tester (QA)** | Tạo Bug; Chuyển trạng thái Epic từ `Active` ➔ `Resolved`; Closed Bug sau khi verify sửa lỗi thành công. |
| **SM / PM** | Giám sát tổng thể quy trình; Closed User Story và Technical Story sau Sprint Review; Closed Epic sau release Production. |

### 6.3 Định Nghĩa Các Loại Work Item & Technical Story

| Work Item | Người tạo | Mô tả chi tiết chức năng |
| :--- | :--- | :--- |
| **Epic** | CPO | Mục tiêu lớn, cấp chiến lược; chứa và gắn nhiều User Story. |
| **User Story** | BA | Yêu cầu cụ thể của người dùng; chứa và gắn nhiều Task và Bug. |
| **Technical Story** | Dev / Team | Các nội dung xử lý kỹ thuật ngoài User Story (ví dụ: test epic, plan release lên production); **cùng cấp với User Story**; chứa nhiều Task và Bug. |
| **Task** | Dev / Tester | Việc cụ thể để hoàn thành một PBI (Product Backlog Item); do Dev hoặc Tester thực hiện. |
| **Bug** | Tester | Lỗi phát hiện trong quá trình test; Dev chịu trách nhiệm sửa. |

### 6.4 Ma Trận Trạng Thái & Người Chuyển Trạng Thái

#### 6.4.1 Epic State Matrix
| Trạng thái | Mô tả chi tiết | Người thực hiện chuyển |
| :--- | :--- | :--- |
| **New** | Mới khởi tạo | CPO |
| **Active** | Chỉ active khi có User Story đầu tiên active | Tự động / CPO |
| **Resolved** | Các User Story và Technical Story con đã Closed, đã test pass ở môi trường Dev | **Chỉ Tester** |
| **Closed** | Đã release chính thức lên Production | CPO / SM / PM |
| **Blocked** | Tạm dừng Epic; khi tiếp tục chuyển về Active | Dev / Team |

#### 6.4.2 User Story State Matrix
| Trạng thái | Mô tả chi tiết | Người thực hiện chuyển |
| :--- | :--- | :--- |
| **New** | Mới khởi tạo | BA |
| **Accepted** | Đã có đầy đủ tài liệu và UI; có thể đưa vào planning và kéo vào sprint | **Chỉ BA** |
| **Active** | Đang trong sprint hiện tại; có ít nhất 1 Task ở trạng thái Active | Dev |
| **Resolved** | Dev hoàn thành theo AC; code có ít nhất 2 người review; các Task/Bug Closed; pass unit test và integration test; đã merge lên môi trường Test | Dev |
| **Closed** | Đã demo trong sprint review; CPO/SM/PM xác nhận đáp ứng AC | CPO / SM / PM |
| **Blocked** | Tạm dừng User Story; khi tiếp tục chuyển về Active | Dev |

> ⚠️ **Lưu ý đặc biệt**: Nếu Sprint Review đánh giá User Story chưa đạt AC, trạng thái được hoàn lại từ **`Resolved` ➔ `Accepted`** để Dev làm lại trong sprint kế tiếp.

#### 6.4.3 Technical Story State Matrix
| Trạng thái | Mô tả chi tiết | Người thực hiện chuyển |
| :--- | :--- | :--- |
| **New** | Mới khởi tạo, mô tả nội dung kỹ thuật cần thực hiện | Dev / Team |
| **Active** | Đang trong sprint hiện tại; có ít nhất 1 Task hoặc Bug ở trạng thái Active | Dev |
| **Resolved** | Tất cả Task và Bug con đã ở trạng thái Closed | Tester / Dev |
| **Closed** | Được xác nhận trong sprint review | CPO / SM / PM |
| **Blocked** | Tạm dừng; khi tiếp tục chuyển về Active | Dev |

#### 6.4.4 Task State Matrix
| Trạng thái | Mô tả chi tiết | Người thực hiện chuyển |
| :--- | :--- | :--- |
| **New** | Khởi tạo, nhập title và mô tả | Dev |
| **Active** | Đang thực hiện | Dev |
| **Resolved** | Đã hoàn thành, tạo Pull Request (PR) chờ review | Dev |
| **Closed** | Đã được review và approve PR thành công | Reviewer / Dev |
| **Blocked** | Tạm dừng | Dev |

> ⚠️ **Lưu ý đặc biệt**: Sau khi review có yêu cầu điều chỉnh code, Task được chuyển ngược từ **`Resolved` ➔ `Active`**.

#### 6.4.5 Bug State Matrix
| Trạng thái | Mô tả chi tiết | Người thực hiện chuyển |
| :--- | :--- | :--- |
| **New** | Do Tester khởi tạo | Tester |
| **Active** | Dev đang thực hiện fix bug | Dev |
| **Resolved** | Dev đã fix xong; đã merge đầy đủ trên môi trường Test; sẵn sàng để Tester verify | Dev |
| **Closed** | Tester xác nhận bug không còn tồn tại | Tester |

> ⚠️ **Lưu ý đặc biệt**: Nếu Tester verify thấy bug vẫn còn, chuyển Bug từ **`Resolved` ➔ `New`**; Dev kéo lại về `Active` để fix tiếp.

### 6.5 Phân Định Môi Trường, Flow Deploy Code, Branching & CI/CD Pipeline

| STT | Môi trường | Tên gọi mã | Mô tả chức năng |
| :---: | :--- | :--- | :--- |
| 1 | **Local** | Local | Lập trình viên làm việc và test trên máy cá nhân trước khi merge. |
| 2 | **Development** | Dev / Test | Toàn bộ Lập trình viên merge code về đây để test integration & Tester vào test. |
| 3 | **Staging** | Staging / UAT / Preview | Môi trường gần giống Production; dùng cho Tester và BA/CPO kiểm thử chấp nhận trước release. |
| 4 | **Production** | Prod | Môi trường chính thức người dùng thực tế khai thác. Chỉ deploy sau khi pass 100% test ở Staging. |

```text
┌─────────┐    ┌───────────┐    ┌──────────────┐    ┌────────────┐
│  Local  │───►│ Dev/Test  │───►│   Staging/   │───►│ Production │
│         │    │           │    │   UAT test   │    │            │
│ Dev code│    │Unit Test  │    │   release    │    │ Người dùng │
│ locally │    │Integration│    │              │    │ thực tế    │
└─────────┘    └───────────┘    └──────────────┘    └────────────┘
Flow deploy chuẩn: Code Local ──► Development (Dev/Test) ──► Staging (UAT) ──► Production
```

#### 6.5.1 Chiến Lược Quản Lý Branch (Trunk-Based Development + Feature Branches)
Company áp dụng mô hình **Trunk-Based Development kết hợp Short-Lived Feature Branches** để tối ưu hóa tốc độ tích hợp và giảm rủi ro merge conflicts:
- **Trunk Branch (`main`)**: Branch duy nhất đại diện cho trạng thái sẵn sàng phát hành của sản phẩm. Code trên `main` luôn ở trạng thái buildable và có thể deploy.
- **Feature Branch (`feat/<ticket-id>-<short-description>`)**: Tạo ra từ `main` để thực hiện một Task/User Story cụ thể.
  - *Thời gian tồn tại (Lifespan)*: Rất ngắn (**< 2 ngày làm việc**).
  - *Quy tắc Merge*: Phải tạo Pull Request (PR), pass 100% CI Pipeline (build, unit test, SAST) và có **ít nhất 2 người review (Peer Review)** phê duyệt trước khi merge về `main`.
- **Hotfix Branch (`fix/<ticket-id>-<short-description>`)**: Tạo ra từ `main` khi có sự cố khẩn cấp trên Production. Sửa lỗi, test nhanh trên Staging và merge lại về `main` sau khi phê duyệt.

#### 6.5.2 Quy Ước Commit Chuẩn (Conventional Commits Standard)
Toàn bộ commit message bắt buộc tuân thủ chuẩn **Conventional Commits 1.0.0**:
- **Cú pháp chuẩn**: `<type>(<scope>): <description> [Ticket-ID]`
- **Danh sách Types bắt buộc**:
  - `feat`: Tính năng mới cho người dùng.
  - `fix`: Sửa lỗi (bug fix).
  - `docs`: Thay đổi tài liệu, comment.
  - `style`: Thay đổi format, khoảng trắng, không ảnh hưởng logic code.
  - `refactor`: Tái cấu trúc code (không thêm feat, không fix bug).
  - `perf`: Cải thiện hiệu năng (performance optimization).
  - `test`: Thêm hoặc bổ sung unit/integration tests.
  - `build`: Thay đổi hệ thống build, dependencies (cargo, npm,...).
  - `ci`: Thay đổi file cấu hình CI/CD (GitHub Actions, GitLab CI,...).
  - `chore`: Công việc phụ trợ không thuộc các loại trên.
- *Ví dụ chuẩn*: `feat(loyalty): add API endpoint to redeem reward points [US-204]`
- *Ví dụ chuẩn*: `fix(auth): fix token expiration issue on refresh [BUG-109]`

#### 6.5.3 Quy Chuẩn Đánh Phiên Bản (SemVer 2.0.0) & Quản Lý Artifact Registry
- **Semantic Versioning (SemVer 2.0.0)**: Định dạng phiên bản `MAJOR.MINOR.PATCH` (Ví dụ: `v2.4.1`).
  - `MAJOR`: Thay đổi phá vỡ tương thích (Breaking API Changes).
  - `MINOR`: Bổ sung tính năng mới nhưng vẫn tương thích ngược (Backward-compatible feature).
  - `PATCH`: Sửa lỗi tương thích ngược (Backward-compatible bug fixes).
- **Artifact Registry Governance**:
  - Mọi bản build thành công từ CI Pipeline đều được gắn tag SemVer và lưu trữ tự động trên **Private Artifact Registry** (Container Registry / Cargo Registry).
  - Tuyệt đối cấm sử dụng tag `latest` trên môi trường Production; Production bắt buộc deploy đích danh theo Immutable Image Tag (ví dụ: `registry.company.com/loyalty-service:v2.4.1`).

#### 6.5.4 Quy Trình Pipeline CI/CD Tự Động (Automated Pipeline Stages)
Mỗi Pull Request hoặc Commit push lên repository sẽ tự động kích hoạt pipeline 5 giai đoạn:

```text
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ Stage 1:     │───►│ Stage 2:     │───►│ Stage 3:     │───►│ Stage 4:     │───►│ Stage 5:     │
│ Build & Lint │    │ Unit Test &  │    │ Static Security│  │ Staging      │    │ Production   │
│ Formatting   │    │ Coverage     │    │ & Dep Scan   │    │ Deploy & DAST│    │ Approval Gate│
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
```

1. **Stage 1 (Build & Lint)**: Kiểm tra cú pháp, format code và compile dự án.
2. **Stage 2 (Unit Testing & Coverage)**: Thực thi 100% unit tests. Tự động reject nếu Coverage < 80% đối với Business Core layer.
3. **Stage 3 (Static Security & Dependency Scan)**: Chạy SAST (SonarQube/Semgrep) và quét CVE dependencies (Trivy/Cargo audit). Chặn pipeline nếu phát hiện lỗi bảo mật Critical/High.
4. **Stage 4 (Staging Deploy & Integration Tests)**: Tự động deploy lên môi trường Staging và chạy Integration tests / DAST smoke tests.
5. **Stage 5 (Production Deploy with Approval Gate)**: Deploy bản build đã verify ở Staging lên Production. Bắt buộc có nút ấn xác nhận phê duyệt (Approval Gate) từ CPO / Tech Lead / DevOps.

### 6.6 Workflow Chi Tiết Từng Loại Work Item

#### 6.6.1 Workflow Tổng Quan Phân Tầng Work Item

```text
┌─────────────────────────────────────────────────────────────┐
│                         EPIC                                │
│              New → Active → Resolved → Closed               │
└───────────────────────┬─────────────────────────────────────┘
                        │ chứa
          ┌─────────────┴──────────────┐
          ▼                            ▼
┌─────────────────┐          ┌──────────────────────┐
│   USER STORY    │          │   TECHNICAL STORY    │
│ New→Accepted→   │          │  New→Active→         │
│ Active→Resolved │          │  Resolved→Closed     │
│ →Closed         │          │  (Blocked)           │
│ (Blocked)       │          └──────────┬───────────┘
└────────┬────────┘                     │
         │ chứa                         │ chứa
    ┌────┴────┐                    ┌────┴────┐
    ▼         ▼                    ▼         ▼
┌────────┐ ┌────────┐          ┌────────┐ ┌────────┐
│  TASK  │ │  BUG   │          │  TASK  │ │  BUG   │
└────────┘ └────────┘          └────────┘ └────────┘
```

#### 6.6.2 Detailed Epic Workflow
```text
         [CPO tạo]
              │
              ▼
           ┌─────┐
           │ NEW │
           └──┬──┘
              │ User Story đầu tiên Active
              ▼
          ┌────────┐                           │
          │ ACTIVE │◄──────────────────────────┤
          └────┬───┘ ──tạm dừng────────────────►
               │ Tester: tất cả US/TS Closed   │
               │ + test pass ở Dev             │
               ▼                               │
          ┌──────────┐                         │
          │ RESOLVED │                         │
          └────┬─────┘                         │
               │ Sau khi release Production    │
               ▼                               │
          ┌────────┐                           │
          │ CLOSED │                           │
          └────────┘                           │
                                               │
          ┌─────────┐  khi tiếp tục            │
          │ BLOCKED │──────────────────────────┘
          └─────────┘
```
- CPO khởi tạo Epic với trạng thái `New`, mô tả mục tiêu chiến lược.
- Epic tự động hoặc được chuyển sang `Active` khi User Story đầu tiên trong Epic được kéo vào sprint và chuyển sang `Active`.
- Epic được Tester chuyển sang `Resolved` khi: (1) Tất cả User Story và Technical Story con đã ở trạng thái `Closed`; (2) Đã test pass trên môi trường Dev.
- Epic được chuyển sang `Closed` bởi CPO/SM/PM sau khi toàn bộ nội dung đã được release lên Production.
- Epic có thể chuyển sang `Blocked` từ `Active` khi cần tạm dừng, và quay lại `Active` khi vướng mắc được giải quyết.

#### 6.6.3 Detailed User Story Workflow
```text
         [BA tạo]
              │
              ▼
           ┌─────┐
           │ NEW │
           └──┬──┘
              │ BA: đủ docs + UI
              ▼
        ┌──────────┐
        │ ACCEPTED │◄──────────────────────────────┐
        └────┬─────┘                               │ Sprint review chưa đạt AC
             │ Dev: kéo vào sprint                 │
             ▼                                     │
          ┌────────┐  tạm dừng ──────►  ┌─────────┐
          │ ACTIVE │                     │ BLOCKED │
          └────┬───┘  ◄── khi tiếp tục  └─────────┘
               │ Dev: xong AC + 2 review + Task/Bug Closed
               │ + pass test + merge lên Test env
               ▼
          ┌──────────┐
          │ RESOLVED │
          └────┬─────┘
               │ CPO/SM/PM: demo OK + đạt AC
               ▼
          ┌────────┐
          │ CLOSED │
          └────────┘
```
- BA tạo User Story với trạng thái `New`, ghi rõ yêu cầu người dùng và Acceptance Criteria (AC).
- BA chuyển sang `Accepted` khi đã có đầy đủ tài liệu kỹ thuật và UI design. **Chỉ những User Story ở trạng thái Accepted mới được đưa vào planning và kéo vào sprint**.
- Dev chuyển sang `Active` khi User Story được kéo vào sprint đang chạy và bắt đầu có Task được thực hiện (ít nhất 1 Task ở Active).
- Dev chuyển sang `Resolved` khi đáp ứng đồng thời tất cả điều kiện:
  1. Hoàn thành đầy đủ theo AC đã mô tả.
  2. Code đã được ít nhất 2 người review (người thực hiện + 1 thành viên khác trong team).
  3. Tất cả Task và Bug trong User Story đều ở trạng thái `Closed`.
  4. Pass Unit Test và Integration Test.
  5. Đã merge lên môi trường Test.
- CPO/SM/PM chuyển sang `Closed` sau sprint review khi đã demo và xác nhận đáp ứng đúng AC.
- Nếu sprint review đánh giá chưa đạt AC, trạng thái được hoàn lại về `Accepted` để Dev làm lại trong sprint kế tiếp.
- User Story có thể chuyển sang `Blocked` bất cứ lúc nào từ `Active` khi có vướng mắc, và quay lại `Active` khi vướng mắc được giải quyết.

#### 6.6.4 Detailed Technical Story Workflow
```text
      [Dev / Team tạo]
              │
              ▼
           ┌─────┐
           │ NEW │
           └──┬──┘
              │ Dev: kéo vào sprint
              ▼
          ┌────────┐  tạm dừng ──────►  ┌─────────┐
          │ ACTIVE │                     │ BLOCKED │
          └────┬───┘  ◄── khi tiếp tục  └─────────┘
               │ Tester/Dev: tất cả Task/Bug Closed
               ▼
          ┌──────────┐
          │ RESOLVED │
          └────┬─────┘
               │ CPO/SM/PM: xác nhận trong sprint review
               ▼
          ┌────────┐
          │ CLOSED │
          └────────┘
```
- Dev/Team khởi tạo Technical Story với trạng thái `New`, mô tả rõ nội dung kỹ thuật cần thực hiện (ví dụ: test epic, plan release, migration,...).
- Dev chuyển sang `Active` khi Technical Story được kéo vào sprint và có ít nhất 1 Task hoặc Bug ở trạng thái `Active`.
- Tester hoặc Dev chuyển sang `Resolved` khi tất cả Task và Bug trong Technical Story đều ở trạng thái `Closed`.
- CPO/SM/PM chuyển sang `Closed` sau khi được xác nhận trong sprint review.
- Technical Story có thể chuyển sang `Blocked` từ `Active` khi cần tạm dừng, và quay lại `Active` khi vướng mắc được giải quyết.

#### 6.6.5 Detailed Task Workflow
```text
         [Dev tạo]
              │
              ▼
           ┌─────┐
           │ NEW │
           └──┬──┘
              │ Dev bắt đầu làm
              ▼
          ┌────────┐        tạm dừng     ┌─────────┐
          │ ACTIVE │◄───────────────────►│ BLOCKED │
          └────┬───┘                     └─────────┘
               │ Dev hoàn thành
               ▼
          ┌──────────┐
          │ RESOLVED │ ──► cần điều chỉnh ──► ACTIVE
          └────┬─────┘
               │ Review & approve PR
               ▼
          ┌────────┐
          │ CLOSED │
          └────────┘
```
- Dev khởi tạo Task với trạng thái `New`, nhập title và mô tả chi tiết công việc.
- Dev tự chuyển sang `Active` khi bắt đầu thực hiện Task.
- Dev chuyển sang `Resolved` khi hoàn thành công việc và tạo Pull Request (PR) chờ review.
- Sau khi PR được review và approve thành công, Task được chuyển sang `Closed`.
- Nếu review yêu cầu chỉnh sửa, Task được chuyển từ `Resolved` ➔ `Active` để Dev thực hiện điều chỉnh.
- Task có thể chuyển sang `Blocked` khi cần tạm dừng vì phụ thuộc bên ngoài.

#### 6.6.6 Detailed Bug Workflow
```text
       [Tester tạo]
              │
              ▼
           ┌─────┐
           │ NEW │◄────────────────────────────────┐
           └──┬──┘                                 │ Tester: verify còn bug
              │ Dev nhận bug                        │
              ▼                                     │
          ┌────────┐                          ┌──────────┐
          │ ACTIVE │─── Dev fix xong ───────►│ RESOLVED │
          └────────┘                          └────┬─────┘
                                                   │ Tester: verify đã hết bug
                                                   ▼
                                              ┌────────┐
                                              │ CLOSED │
                                              └────────┘
```
- Tester khởi tạo Bug với trạng thái `New`, mô tả lỗi, môi trường phát sinh, và các bước tái hiện.
- Dev chuyển sang `Active` khi bắt đầu điều tra và fix bug.
- Dev chuyển sang `Resolved` khi: (1) Đã fix xong bug; (2) Đã merge đầy đủ code lên môi trường Test; (3) Sẵn sàng cho Tester verify.
- Tester chuyển sang `Closed` sau khi verify và xác nhận bug không còn tồn tại.
- Nếu Tester verify thấy bug vẫn còn, chuyển Bug từ `Resolved` ➔ `New`; Dev kéo lại về `Active` để fix tiếp.

---

## 7. KIẾN TRÚC & THIẾT KẾ DOMAIN-DRIVEN DESIGN (DDD) VÀ THIẾT LẬP MÔ HÌNH TEAM OWNERSHIP TẠI COMPANY

### 7.1 Tổng Quan Kiến Trúc Microservices tại Company
- **Định nghĩa Kiến trúc**: Microservices tại Company là kiến trúc phần mềm được thiết kế nhằm chia ứng dụng lớn thành nhiều dịch vụ nhỏ, độc lập và triển khai riêng biệt.
- **Sự khác biệt với Monolith truyền thống**: Mỗi Microservice chỉ đảm nhiệm một chức năng nghiệp vụ chuyên biệt (ví dụ: quản lý người dùng, xử lý thanh toán, theo dõi đơn hàng) và có thể được phát triển, kiểm thử, triển khai hoặc nâng cấp độc lập mà không làm gián đoạn toàn bộ hệ thống.
- **3 Thành phần chính của Microservices Company**:
  1. **Services**: Các dịch vụ riêng lẻ đảm nhiệm từng chức năng cụ thể.
  2. **API Gateway**: Cổng kết nối trung tâm giữa các dịch vụ, điều phối và quản lý các yêu cầu từ người dùng đến các dịch vụ khác nhau, đảm bảo giao tiếp thông suốt và bảo mật.
  3. **Database riêng cho từng dịch vụ**: Mỗi dịch vụ sở hữu cơ sở dữ liệu riêng (Database-per-Service) để đảm bảo tính độc lập và tuyệt đối tránh xung đột dữ liệu.
- **Nguyên tắc phát triển**: Bắt đầu một hệ thống Microservices phải **bắt đầu từ nghiệp vụ (Domain)** và tổ chức phát triển thành hệ thống các services tương ứng.

### 7.2 Thiết Kế Hệ Thống Theo Miền Nghiệp Vụ (Domain & Bounded Context)
- **Định nghĩa Domain (Miền nghiệp vụ)**: Domain là một khả năng kinh doanh cụ thể hoặc một lĩnh vực chức năng cụ thể (ví dụ: Quản lý kho, Quản lý thanh toán). Domain xác định ranh giới của logic nghiệp vụ và dữ liệu, đồng thời là tập hợp các Microservices có liên quan chặt chẽ với nhau.
- **Vai trò của Domain trong Microservices tại Company**:
  - **Xác định ranh giới (Bounded Context)**: Mỗi domain đại diện cho một ranh giới nghiệp vụ độc lập. Mỗi Microservice sẽ chịu trách nhiệm triển khai logic cho đúng một domain duy nhất.
  - **Tính toàn vẹn dữ liệu**: Mỗi domain sở hữu cơ sở dữ liệu riêng chứa các thực thể (Entity) và quy tắc nghiệp vụ hoàn chỉnh, giúp các Microservice hoạt động độc lập và giảm tối đa sự phụ thuộc chéo.

#### Bảng Ví Dụ Quản Lý Các Domain Cốt Lõi Tại Company:
| Domain Name | Chức năng nghiệp vụ chính |
| :--- | :--- |
| **Identity & Access** | Đăng nhập, xác thực, phân quyền người dùng và hệ thống. |
| **Customer** | Quản lý thông tin hồ sơ khách hàng và tương tác. |
| **Loyalty** | Tích điểm, quản lý hạng thành viên và đổi quà. |
| **Promotion** | Quản lý chương trình khuyến mãi, mã giảm giá và voucher. |
| **Order / Checkout** | Quản lý đơn hàng, giỏ hàng và thanh toán. |
| **Inventory** | Quản lý tồn kho, nhập xuất và điều chuyển kho. |

### 7.3 Cấu Trúc Phân Tầng Kiến Trúc (Architecture Layers Topology)
Hệ thống kiến trúc tại Company phân rã các Domain thành 5 Tầng kiến trúc (Layers) có cùng vai trò:

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ 1. EXPERIENCE LAYER                                                     │
│ (Sản phẩm UI/App người dùng: TPOS, WiOn POS, WiOn Dental,...)           │
├─────────────────────────────────────────────────────────────────────────┤
│ 2. DATA & INTELLIGENCE LAYER (Cross-cutting Layer)                      │
│ (Thu thập dữ liệu, phân tích, dự đoán, hỗ trợ quyết định)               │
├─────────────────────────────────────────────────────────────────────────┤
│ 3. BUSINESS CORE LAYER                                                  │
│ (Năng lực kinh doanh cốt lõi tạo giá trị trực tiếp: Loyalty, Promotion) │
├─────────────────────────────────────────────────────────────────────────┤
│ 4. ENTERPRISE CORE LAYER                                                │
│ (Năng lực doanh nghiệp dùng chung: Customer, Order, Identity)            │
├─────────────────────────────────────────────────────────────────────────┤
│ 5. PLATFORM FOUNDATION LAYER                                            │
│ (Hạ tầng kỹ thuật, API Gateway, Auth Engine, Logging, Service Mesh)     │
└─────────────────────────────────────────────────────────────────────────┘
```

1. **Platform Foundation Layer**: Cung cấp các năng lực nền tảng kỹ thuật và hạ tầng dùng chung cho toàn bộ hệ sinh thái (Gateway, Auth, Infrastructure).
2. **Enterprise Core Layer**: Các năng lực doanh nghiệp dùng chung cho hầu hết sản phẩm và ngành nghề.
3. **Business Core Layer**: Chứa các năng lực kinh doanh cốt lõi tạo ra giá trị trực tiếp cho khách hàng.
4. **Data & Intelligence Layer (Cross-cutting Layer)**: Biến dữ liệu thành thông tin, tri thức và hành động thông minh. Đặc điểm: Thu thập dữ liệu từ Business Core & Enterprise Core; Phân tích, dự đoán, hỗ trợ ra quyết định.
5. **Experience Layer**: Các sản phẩm, ứng dụng và giao diện người dùng cuối trực tiếp khai thác sử dụng (VD: TPOS, WiOn POS, WiOn Dental,...).

### 7.4 Thiết Lập Mô Hình Team Ownership & Domain Ownership

```text
Mô hình Phân Phối Ownership:
  Team A (Chuyên gia Khách hàng)  ──► Sở hữu & Phụ trách ──► Customer Service (Domain Customer)
  Team B (Chuyên gia Đơn hàng)    ──► Sở hữu & Phụ trách ──► Order Service (Domain Order)
```

- **Sự tự chủ của các nhóm (Cross-functional Teams)**: Mỗi Microservice được phát triển, triển khai và vận hành bởi một nhóm độc lập. Nhóm có toàn quyền quyết định từ công nghệ, thiết kế đến quy trình phát hành.
- **Trách nhiệm của Team Owner Domain**:
  1. **Domain Knowledge**: Sở hữu tri thức và tài liệu chuẩn của Domain.
  2. **API**: Thiết kế và quản lý API endpoints công khai.
  3. **Event**: Quản lý Event Catalog và các sự kiện phát ra.
  4. **Data Model**: Quản lý Entity, Schema DB của Domain.
  5. **Technical Roadmap**: Lộ trình nâng cấp kỹ thuật của Domain.
- **Giao tiếp giảm thiểu (Reduced Communication Overhead)**: Giao tiếp giữa các nhóm được thực hiện thông qua tài liệu công khai và API/Event Contracts rõ ràng. Các nhóm chỉ quan tâm đến đầu ra (Output) của service khác.
- **Quyền sở hữu mã nguồn (Code Ownership)**: Mỗi Microservice phải thuộc về **duy nhất một nhóm** để loại bỏ tình trạng "cha chung không ai khóc" và loại bỏ tranh chấp khi sửa code.
- **Thay đổi văn hóa (Cultural Shift)**: Phân quyền, tự tin tưởng, áp dụng tự động hóa kiểm thử và CI/CD độc lập.

### 7.5 Nguyên Tắc Giao Tiếp & Minh Bạch Thông Tin Giữa Các Nhóm
Để giải quyết bài toán phân mảnh thông tin khi nhiều nhóm cùng tích hợp (VD: Team Loyalty làm việc với Team TPOS):
1. **Single Source of Truth**: Thông tin được lưu tại một nơi công khai mọi người đều xem được. Team Owner là "single source of truth" (Outline Wiki / Confluence). Tránh việc mỗi team hiểu theo một kiểu riêng.
2. **Giao tiếp theo Kênh Domain**: Tạo kênh giao tiếp chính thức theo định dạng `#domain-<name>` (ví dụ: `#domain-loyalty`) để thông báo API mới, Breaking Changes, Roadmap,... Tuyệt đối không tạo các group chat riêng lẻ phân mảnh.
3. **Contract-First Integration**:
   - Khi cần tích hợp, các team tự đọc tài liệu API/Event Contract công khai mà không cần mở nhóm hỏi lại mỗi lần.
   - Các team phát triển code chuẩn theo Contract. Khi có thay đổi phải thông báo trước trên kênh Domain.

### 7.6 Quản Lý Domain Knowledge & Lưu Trữ Tài Liệu Trực Tuyến
- **Phân biệt Feature Analysis vs Domain Knowledge**:
  - *Feature Analysis (Use Case)*: Giải quyết nhu cầu kinh doanh cụ thể của 1 sản phẩm.
  - *Domain Knowledge*: Kiến thức chuẩn hóa của Domain dùng chung.
- **Thành phần cấu thành Domain Knowledge**:
  1. **Business Glossary**: Khái niệm nghiệp vụ chuẩn.
  2. **Domain Model**: Định nghĩa các đối tượng nghiệp vụ.
  3. **Domain Rules**: Các quy tắc nghiệp vụ cố định.
  4. **Domain Flow**: Luồng xử lý nghiệp vụ chuẩn.
  5. **Event Catalog**: Danh mục sự kiện hệ thống.
  6. **API Catalog**: Danh mục các API endpoints.
  7. **Decision Records (ADR)**: Giải thích lý do ra quyết định (VD: *Tại sao điểm tích lũy hết hạn sau 365 ngày?*).
  8. **Integrated Documentation**: Tài liệu hướng dẫn tích hợp cho các team khác.
- **Lưu trữ Domain Knowledge tại Company**: Lưu trữ trực tuyến trên **Outline Wiki** tại thư mục: `Tài liệu theo Domain > #<Domain_Name> Domain` (Ví dụ: `Tài liệu theo Domain > #Loyalty Domain`).
- **Quy tắc về UI**: Domain Knowledge không bao gồm mô tả thiết kế UI chi tiết, nhưng chứa các quy tắc nghiệp vụ ảnh hưởng trực tiếp đến hiển thị UI (Ví dụ: *Membership Tier Display Rule: VIP ➔ Badge VIP, GOLD ➔ Color Gold*).

### 7.7 Quy Định Về UI Front-End, UI Widgets & UI Fragments Dùng Chung
- **Quyền sở hữu UI FE**:
  - Nếu UI thuộc trải nghiệm người dùng của sản phẩm cụ thể ➔ **Team Sản phẩm (Product Team)** chịu trách nhiệm thực hiện.
  - Nếu UI là thành phần dùng chung nhiều sản phẩm (Domain UI Widget / Fragment) ➔ **Team Domain** chịu trách nhiệm thực hiện.
- **Khái niệm**:
  - **UI Widget**: Component UI có nghiệp vụ độc lập, không phụ thuộc UI khác (VD: UI Notification, Badge điểm,...).
  - **UI Fragment**: Layout UI phức tạp gồm nhiều Widget, xử lý nhiều nghiệp vụ.
- **Quản lý UI dùng chung**: Được tổ chức và quản lý bởi **Team UI/UX**. Có 1 file lưu trữ chuẩn các Widgets/Fragments dùng chung cho nhiều dự án. Chỉ quản lý UI trên dự án sản phẩm, không lưu trữ UI lẻ tẻ trên Domain.

### 7.8 Phân Định Trách Nhiệm Kiểm Thử (Product Testing vs Domain Testing)
Cả hai team đều bắt buộc phải test, nhưng phạm vi phân định rõ ràng:

| Đội ngũ Tester | Trách nhiệm kiểm thử chính |
| :--- | :--- |
| **Domain Team Tester** | Chịu trách nhiệm đảm bảo **Domain hoạt động đúng**: Test Domain Rules, Test API contract, Test Event Catalog và hiệu năng service. |
| **Product Team Tester** | Chịu trách nhiệm về **tính năng cuối cùng của người dùng**: Test trải nghiệm end-to-end, Test tính năng sản phẩm trên giao diện, UAT. |

### 7.9 Quy Trình Xử Lý Thay Đổi Phá Vỡ (Breaking Change)
Khi Domain Team cần chỉnh sửa API/Event gây ảnh hưởng đến các team đang tích hợp (Breaking Change):
1. **Lộ trình nâng cấp**: Tuân thủ lộ trình chuyển đổi: `v1 ➔ v2 ➔ Migration Window ➔ Deprecate`.
2. **Quy tắc duy trì**: Domain Team phát hành API v2 nhưng **bắt buộc phải duy trì phiên bản v1 chạy song song** trong suốt thời gian Migration Window cho đến khi tất cả các Product Team tích hợp hoàn tất nâng cấp lên v2.

### 7.10 Phân Bổ Tỷ Lệ Ưu Tiên Product Backlog & Quyền Quyết Định Business Rule
- **Tỷ lệ phân bổ năng lực Backlog chuẩn tại Company**:
  - **70%**: Product Features & Enhancements (Yêu cầu sản phẩm).
  - **20%**: Domain Improvements (Cải tiến Năng lực Domain).
  - **10%**: Technical Debt & Infrastructure (Nợ kỹ thuật & Hạ tầng).
- **Quyền quyết định Business Rule khi có bất đồng**:
  1. Team Product có quyền đề xuất và chứng minh nhu cầu/giá trị kinh doanh với Domain Rule mới.
  2. Domain Owner đánh giá tác động lên toàn bộ các dự án khác ➔ Ra quyết định Domain Rule cuối cùng.
  3. Trong trường hợp không thống nhất được (Product muốn tạo rule riêng chỉ áp dụng cho sản phẩm của mình) ➔ **CPO sẽ họp và đưa ra quyết định thống nhất cuối cùng**.

### 7.11 Phân Định Quyền Hạn Chi Tiết (Product Team vs Domain Team vs CPO)

```text
┌─────────────────────────┐  ┌─────────────────────────┐  ┌─────────────────────────┐
│      PRODUCT TEAM       │  │       DOMAIN TEAM       │  │           CPO           │
├─────────────────────────┤  ├─────────────────────────┤  ├─────────────────────────┤
│ • Đề xuất yêu cầu       │  │ • Đánh giá giải pháp    │  │ • Giải quyết xung đột   │
│ • Mô tả Business Value  │  │ • Ước lượng effort      │  │   ưu tiên cross-team    │
│ • Đề xuất mức độ ưu tiên│  │ • Thiết kế Domain       │  │ • Phân bổ nguồn lực     │
│ • Sở hữu Trải nghiệm    │  │ • Quản lý Domain Backlog│  │ • Quyết định Roadmap    │
│   (Experience)          │  │ • Sở hữu Năng lực       │  │   liên sản phẩm         │
└─────────────────────────┘  └─────────────────────────┘  └─────────────────────────┘
```

### 7.12 Ma Trận Phân Công Công Việc Tích Hợp Thực Tế (Ví dụ: TPOS tích hợp Loyalty)
Trường hợp cụ thể khi Team TPOS (Product) muốn triển khai tính năng Khuyến mãi/Tích điểm liên quan đến Team Loyalty (Domain):

| Hạng mục công việc | Team TPOS (Product Team) | Team Loyalty (Domain Team) |
| :--- | :--- | :--- |
| **BA** | • Đề xuất nhu cầu kinh doanh.<br>• Viết User Story.<br>• Viết tài liệu Use Case trên TPOS. | • Phân tích tác động lên Domain.<br>• Ước lượng công việc.<br>• Viết tài liệu Domain Knowledge & Hướng dẫn tích hợp. |
| **UI/UX** | Thiết kế giao diện trải nghiệm sản phẩm TPOS. | Cung cấp thiết kế chuẩn cho các Domain UI Widgets / Fragments dùng chung. |
| **Dev** | • Code UI TPOS.<br>• Code tích hợp gọi API/Event. | • Xác nhận năng lực đáp ứng.<br>• Viết code xử lý API, Event, Logic nghiệp vụ, Micro-frontend. |
| **Tester** | • Test tích hợp, Test tính năng end-to-end.<br>• Chịu trách nhiệm chất lượng Feature cuối cùng trên TPOS. | • Domain Testing: Test Business Rules, Test API, Test Event.<br>• Hỗ trợ xử lý lỗi phát sinh thuộc phạm vi Domain. |
| **Hỗ trợ khách hàng (Support Flow)** | **Customer Support ➔ Product Team ➔ Domain Team** (Theo thứ tự tiếp nhận leo thang). |

### 7.13 Các Nguyên Tắc Vàng (Golden Rules) Trong Kiến Trúc Company
1. **Duy nhất một Owner**: Chỉ có duy nhất 1 Team Owner cho mỗi Domain.
2. **Quyền sở hữu Feature**: Feature thuộc sản phẩm nào thì Product Team sản phẩm đó là Owner.
3. **Phân định Năng lực vs Trải nghiệm**: Product Team sở hữu trải nghiệm người dùng (Experience); Domain Team sở hữu năng lực nghiệp vụ (Capability).
4. **Bảo vệ Contract**: Domain Owner sở hữu toàn bộ Contract (API Docs, Event Catalog). **Không có bất kỳ team nào ngoài Domain Owner được phép tự ý sửa đổi Contract**.

---

## 8. TỔNG HỢP DANH MỤC HỒ SƠ & TIÊU CHUẨN LƯU TRỮ TUÂN THỦ ISO/IEC 27001:2022

Bảng tổng hợp toàn bộ hồ sơ vận hành phần mềm bắt buộc phải được lưu trữ tuân thủ theo Quy chế Quản lý An toàn Thông tin và ISO/IEC 27001:2022:

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                   BẢNG TỔNG HỢP HỒ SƠ QUẢN LÝ PHÁT TRIỂN PHẦN MỀM                     │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

| STT | Tên Hồ Sơ / Biểu Mẫu | Mã Quy Trình | Bộ Phận Trách Nhiệm Lưu | Thời Gian Lưu Trữ Bắt Buộc |
| :---: | :--- | :---: | :---: | :---: |
| **1** | Biên bản tiếp nhận ý tưởng, yêu cầu | SM-QT-01 | BA | **1 năm** |
| **2** | Biên bản đánh giá định hướng ý tưởng, yêu cầu | SM-QT-01 | BA | **1 năm** |
| **3** | Kế hoạch Roadmap Final | SM-QT-01 | BA | **3 năm** |
| **4** | Report Sprint planning meeting | SM-QT-02 | Scrum Master | **1 năm** |
| **5** | Report Sprint Review meeting | SM-QT-02 | Scrum Master | **1 năm** |
| **6** | Output Request Form (ORF) | SM-QT-04 | Tenant / Provider BA | **3 năm** |
| **7** | Biên bản 2-team meeting cross-team | SM-QT-04 | Provider BA | **3 năm** |
| **8** | Kế hoạch Release (Release Plan) | SM-QT-03 / SM-QT-04 | BA / Product Owner | **3 năm** |
| **9** | Kế hoạch Test (Test Plan) | SM-QT-03 / SM-QT-04 | Tester | **3 năm** |
| **10**| Báo cáo kiểm thử phát hành (Test Release Report) | SM-QT-03 / SM-QT-04 | Tester | **3 năm** |
| **11**| Output Document (API Docs, Schema, ERD) | SM-QT-04 / DDD-COMPANY | Tenant Dev / Domain Team | **3 năm** |
| **12**| Product Backlog (Epic, Feature, User Story) | SM-QĐi-01 / SM-QĐ-002 | BA / CPO | **2 năm** (TFS/Jira) |
| **13**| Technical Story Backlog | SM-QĐ-002 | Dev / Team | **2 năm** (TFS) |
| **14**| Sprint Backlog & Task Breakdown | SM-QT-02 / SM-QĐi-01 | Dev Team / SM | **1 tháng** |
| **15**| Danh sách Task & Bug | SM-QĐi-01 / SM-QĐ-002 | Developers / Tester | **1 năm** |
| **16**| Danh sách Testcase | SM-QĐi-01 | Tester | **1 năm** |
| **17**| Domain Knowledge (Glossary, Models, Rules, Event Catalog) | DDD-COMPANY-2026 | Domain Team Owner | **Lưu trữ vĩnh viễn trên Outline Wiki (`#Domain`)** |
| **18**| Tài liệu mô tả tính năng & hướng dẫn sử dụng | SM-QT-03 / SM-QT-04 | Tester | **Lưu đến khi tài liệu không còn chính xác với thực tế sản phẩm** |
| **19**| Báo cáo phân tích sự cố & Blameless Postmortem (RCA Report) | SM-QT-05 / ISO A.5.26 | Dev / DevOps / Tech Lead | **3 năm** |
| **20**| Nhật ký trực On-call & Lịch sử Incident Alerts | SM-QT-05 / ISO A.5.24 | Tech Lead / Operations | **3 năm** |
| **21**| Báo cáo kiểm tra bảo mật (SAST/DAST & Dependency Scan Audit) | SM-QT-06 / ISO A.8.25 | Security Lead / DevOps | **3 năm** |
| **22**| Hồ sơ đánh giá tác động dữ liệu cá nhân (DPIA PDPD 13/2023) | SM-QT-06 / PDPD-13 | Compliance Officer / DPO | **5 năm** |
| **23**| Biên bản kiểm thử diễn tập khôi phục thảm họa (DR Restore Test Log) | SM-QT-05 / ISO A.8.13 | Infrastructure Lead / DevOps | **3 năm** |
| **24**| Báo cáo đo lường DORA Metrics & Engineering Performance | SM-QT-07 | Engineering Director / SM | **3 năm (Tự động từ Harness DB)** |
| **25**| Nhật ký giám sát & Audit Log thao tác của AI Agent | SM-QT-08 | System Architect / Lead Dev | **3 năm (Harness Observability)** |

---

## 9. QUY TRÌNH QUẢN LÝ SỰ CỐ HỆ THỐNG VÀ BẢO ĐẢM HOẠT ĐỘNG LIÊN TỤC (INCIDENT MANAGEMENT & DISASTER RECOVERY - ISO/IEC 27001:2022 A.5.24 - A.5.29, A.8.13)

### 9.1 Ma Trận Mức Độ Nghiêm Trọng (Severity Matrix P1-P4) & Cam Kết SLA
Mọi sự cố vận hành trên môi trường Production phải được phân loại ngay lập tức theo ma trận Severity và tuân thủ SLA phản hồi/xử lý:

| Mức độ (Severity) | Định nghĩa & Dấu hiệu nhận biết | SLA Phản hồi (Response SLA) | SLA Xử lý / Tạm thời (Resolution SLA) | Người chủ trì (Incident Commander) |
| :---: | :--- | :---: | :---: | :---: |
| **P1 - Critical** | **Sự cố thảm họa**: Hệ thống ngừng hoạt động hoàn toàn, sập server (OverRAM, OOM Kill, High CPU 100%), mất dữ liệu, hoặc tính năng thanh toán/đăng nhập bị ngắt toàn bộ. | **≤ 15 phút** | **≤ 2 giờ** | CTO / Lead DevOps |
| **P2 - High** | **Sự cố nghiêm trọng**: Tính năng cốt lõi bị gián đoạn (VD: không tạo được đơn hàng), ảnh hưởng > 20% lượng người dùng nhưng có phương án tạm thời (Workaround). | **≤ 30 phút** | **≤ 4 giờ** | Tech Lead / Domain Owner |
| **P3 - Medium** | **Sự cố trung bình**: Lỗi tính năng phụ, ảnh hưởng < 5% người dùng, có workaround, hệ thống cốt lõi vẫn vận hành bình thường. | **≤ 2 giờ** | **≤ 24 giờ** | Senior Developer / QA Lead |
| **P4 - Low** | **Sự cố nhỏ / Cải tiến**: Lỗi hiển thị UI nhỏ, sai chính tả, chậm không đáng kể, gợi ý tối ưu. | **≤ 8 giờ** | Đưa vào Sprint Backlog kế tiếp | Developer assigned |

### 9.2 Kênh Cảnh Báo (Alerting Channels) & Quy Trình Trực On-Call 24/7
- **Tự động hóa cảnh báo**: Hệ thống giám sát (Prometheus/Grafana/PagerDuty) tự động quét chỉ số hệ thống theo thời gian thực (5s/lần). Khi vượt ngưỡng threshold (RAM > 85%, CPU > 90%, HTTP 5xx Rate > 2%), hệ thống tự động đẩy alert trực tiếp vào kênh chuyên dụng `#incident-alerts` (Slack/Teams/Telegram) và gọi điện tự động (PagerDuty/Opsgenie) tới Kỹ sư On-call.
- **Lịch trực On-call 24/7**: Bộ phận Infrastructure & DevOps thiết lập lịch trực luân phiên 24/7.
- **Quy trình Escalate (Leo thang)**:
  - Nếu sự cố P1/P2 không được Kỹ sư On-call ấn xác nhận (Acknowledge) trong vòng **10 phút** ➔ Cảnh báo tự động đẩy lên Tech Lead.
  - Nếu sau **15 phút** chưa có phản hồi ➔ Cảnh báo tự động gọi điện trực tiếp tới CTO.

### 9.3 Quy Trình Phân Tích Nguyên Nhân Gốc Rễ (RCA) & Blameless Postmortem Trong 48 Giờ
- **Nguyên tắc Blameless (Không đổ lỗi)**: Buổi họp Postmortem tập trung vào việc tìm lỗi của quy trình, công cụ, hạ tầng và giải pháp ngăn ngừa, tuyệt đối không quy trách nhiệm cá nhân.
- **Thời hạn tổ chức**: Bắt buộc tổ chức cuộc họp Blameless Postmortem trong vòng **48 giờ làm việc** sau khi sự cố P1 hoặc P2 được khắc phục hoàn toàn.
- **Nội dung Báo cáo RCA (Root Cause Analysis Report)**:
  1. **Timeline sự cố**: Chi tiết từng mốc thời gian từ khi phát sinh lỗi ➔ nhận alert ➔ khoanh vùng ➔ phát hành bản vá/roll back.
  2. **Nguyên nhân gốc rễ (5 Whys)**: Áp dụng phương pháp hỏi 5 lần "Tại sao" để tìm ra nguyên nhân sâu xa (ví dụ: *RAM tràn ➔ do Memory Leak trong loop ➔ do thiếu bounds check ➔ do PR review bỏ qua ➔ do thiếu load test trước release*).
  3. **Action Items khắc phục lâu dài**: Ghi nhận các Task cải tiến (thêm alert, refactor code, thêm integration test) kèm deadline và người chịu trách nhiệm, đưa trực tiếp vào Sprint Backlog ưu tiên cao nhất.

### 9.4 Giám Sát Sau Release (Post-Release Monitoring) & Ngăn Ngừa Sự Cố Giới Hạn Tài Nguyên
- **Khung thời gian giám sát trọng điểm**: 48 giờ làm việc liên tục sau mỗi đợt Release Production là giai đoạn "Cảnh báo cao độ" (High Vigilance Window).
- **Chỉ số bắt buộc theo dõi (Golden Signals)**:
  1. **Latency**: Thời gian phản hồi trung bình và p99.
  2. **Traffic**: Số lượng request/giây (RPS).
  3. **Errors**: Tỷ lệ lỗi HTTP 5xx và Unhandled Exceptions.
  4. **Saturation**: Mức độ tiêu tốn tài nguyên (RAM, CPU, Disk I/O, DB Connection Pool).
- **Ngăn ngừa sự cố quá tải RAM / Memory Leak (OOM Prevention)**:
  - Bắt buộc thiết lập `Memory Limits` & `CPU Requests/Limits` chuẩn trên từng Container / Kubernetes Pod.
  - Cấu hình tự động `Auto-kill` & `Self-healing Restart` khi Heap Memory chạm ngưỡng 90%.
  - Tự động kích hoạt **Rollback tự động (Automated Canary Rollback)** về phiên bản ổn định trước đó nếu Tỷ lệ Lỗi vượt quá 2% trong 10 phút đầu sau Release.

### 9.5 Kế Hoạch Khôi Phục Sau Thảm Họa (Disaster Recovery - DR) & BCP (ISO A.8.13, A.5.29)
- **Mục tiêu RPO & RTO theo phân tầng dịch vụ**:

| Phân tầng Dịch vụ | Phạm vi ứng dụng | RPO (Recovery Point Objective - Mất dữ liệu tối đa) | RTO (Recovery Time Objective - Thời gian gián đoạn tối đa) |
| :--- | :--- | :---: | :---: |
| **Tier 1 - Critical Core** | Auth, Identity, Payment, Database Prod chính | **≤ 15 phút** | **≤ 1 giờ** |
| **Tier 2 - High Core** | Order, Inventory, Loyalty, Promotion | **≤ 1 giờ** | **≤ 4 giờ** |
| **Tier 3 - Standard** | Analytics, Reporting, Logging, Dev/Staging | **≤ 24 giờ** | **≤ 24 giờ** |

- **Lịch Backup & Diễn tập Khôi phục (Backup & Restore Test Schedule)**:
  - **Full Backup DB**: Thực hiện tự động hằng ngày vào lúc 00:00 UTC, lưu trữ mã hóa AES-256 trên Multi-Region Cloud Storage.
  - **Transaction Log / WAL Backup**: Thực hiện tự động mỗi 15 phút cho Tier 1.
  - **Kiểm thử khôi phục (Restore Drill)**: Định kỳ **1 tháng / 1 lần**, đội ngũ Infrastructure phải diễn tập khôi phục dữ liệu từ bản backup lên môi trường Sandbox độc lập để đo lường RTO thực tế và kiểm tra tính toàn vẹn dữ liệu. Kết quả lưu vào *Biên bản Restore Test Log*.
- **Kế hoạch duy trì hoạt động kinh doanh (BCP - Business Continuity Plan)**:
  - Cấu hình hạ tầng Multi-AZ / Multi-Region Failover tự động đối với Database và API Gateway.
  - Khi xảy ra sự cố sập Datacenter/Region Cloud chính, hệ thống tự động chuyển hướng Traffic (DNS Failover) sang Region dự phòng trong vòng 15 phút.

---

## 10. QUY ĐỊNH BẢO MẬT HỆ THỐNG VÀ DEVSECOPS (ISO/IEC 27001:2022 A.8 & NGHỊ ĐỊNH PDPD 13/2023/NĐ-CP)

### 10.1 Ma Trận Phân Quyền Truy Cập (Access Control Matrix - ISO A.8.2, A.8.3, A.8.4)
Áp dụng nghiêm ngặt nguyên tắc **Quyền tối thiểu (Least Privilege)** và **Phân tách trách nhiệm (Separation of Duties)**:

| Vai trò (Role) | Source Code (Git) | Môi trường Dev/Test | Môi trường Staging | Môi trường Production | Cơ sở dữ liệu Prod |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Developer** | Read / Push (Feature branch) | Full Access | Read / Deploy Test | No Access | No Access (Chỉ xem Anonymized Log) |
| **Tester (QA)** | Read | Full Access | Full Test Access | Read Only (No Deploy) | No Access |
| **Tech Lead / Domain Owner** | Read / Approve PR Merge | Full Access | Full Access | Read / Approve Release | Read Only (Via VPN + Bastion + MFA) |
| **DevOps / Infra Lead** | Full Admin | Full Access | Full Access | Full Admin Access | Emergency Access (Log Audit 100%) |
| **BA / CPO** | Read (Docs only) | User Access | UAT Access | User Access | No Access |

### 10.2 Quy Trình Quản Lý Bí Mật (Secrets Management & Vault Governance - ISO A.8.24)
- **Tuyệt đối cấm Commit Secrets**: CẤM BẮT BUỘC mọi hành vi hardcode hoặc commit file `.env`, Passwords, API Keys, Private Keys, Database Tokens vào Git Repository (kể cả private repository).
- **Hệ thống Quản lý Secret tập trung**: Sử dụng **HashiCorp Vault** (hoặc AWS Secrets Manager / SOPS) để lưu trữ bí mật. Ứng dụng lấy secret runtime thông qua Environment Variables được inject tự động từ Vault.
- **Tự động quét Secret (Automated Secret Scanning)**: Kích hoạt công cụ TruffleHog / GitGuardian chặn từ bước `git commit` (Pre-commit hook) và trong CI Pipeline. Nếu phát hiện Secret lọt vào commit, pipeline lập tức hủy bỏ và vô hiệu hóa (revoke) secret đó ngay lập tức.

### 10.3 Cổng Kiểm Soát Bảo Mật Tự Động Trong CI/CD (Security Gates - ISO A.8.25, A.8.28)
Tích hợp bảo mật tự động vào quy trình phát triển (Shift-Left Security):

```text
┌─────────────────────────┐    ┌─────────────────────────┐    ┌─────────────────────────┐
│ SAST (SonarQube/Semgrep)│───►│ Dependency Scan (Trivy) │───►│ Container Image Scan    │
│ Quét mã nguồn tĩnh      │    │ Quét lỗ hổng thư viện   │    │ Quét Docker Layer CVEs  │
└─────────────────────────┘    └─────────────────────────┘    └─────────────────────────┘
```

1. **SAST (Static Application Security Testing)**: Quét lỗi bảo mật mã nguồn (OWASP Top 10, SQL Injection, XSS, Buffer Overflow). Chặn build nếu có lỗi `High` hoặc `Critical`.
2. **Dependency Vulnerability Scan**: Tự động quét các gói thư viện phụ thuộc (`cargo audit`, `npm audit`, Snyk). Tự động block PR nếu phát hiện CVE mức độ Critical chưa có bản vá.
3. **Container Image Scan**: Quét các Docker layer bằng Trivy trước khi push vào Registry. Loại bỏ các base image chứa lỗ hổng hệ điều hành chưa được patch.
4. **DAST (Dynamic Application Security Testing)**: Tự động chạy OWASP ZAP scan trên môi trường Staging định kỳ trước đợt phát hành lớn.

### 10.4 Tuân Thủ Quy Định Bảo Vệ Dữ Liệu Cá Nhân Theo Nghị Định 13/2023/NĐ-CP (PDPD Vietnam)
Toàn bộ sản phẩm phần mềm của Company vận hành tại Việt Nam bắt buộc phải tuân thủ Nghị định 13/2023/NĐ-CP về Bảo vệ dữ liệu cá nhân (Personal Data Protection Decree - PDPD):

#### A. Phân loại Dữ liệu Cá nhân (Data Classification)
- **Dữ liệu cá nhân cơ bản**: Họ tên, Ngày sinh, Giới tính, SĐT, Email, Địa chỉ, Hình ảnh cá nhân.
- **Dữ liệu cá nhân nhạy cảm**: Tài khoản ngân hàng, Thông tin thanh toán, Dữ liệu vị trí địa lý, Lịch sử giao dịch tài chính, Dữ liệu sinh trắc học.

#### B. Nguyên tắc Bảo vệ & Mã hóa (Encryption & Storage)
- **Mã hóa dữ liệu**: Bắt buộc mã hóa AES-256 đối với dữ liệu nhạy cảm lưu trữ trong Database (Encryption at Rest) và mã hóa TLS 1.3 cho toàn bộ dữ liệu truyền tải trên mạng (Encryption in Transit).
- **Phân tách DB Dữ liệu cá nhân**: Lưu trữ dữ liệu định danh khách hàng tại Database riêng biệt, áp dụng Pseudonymization (giả danh hóa) hoặc Anonymization (ẩn danh hóa) khi trích xuất làm dữ liệu kiểm thử hoặc phân tích data.

#### C. Vòng đời Dữ liệu & Quyền người dùng (Data Retention & Right to be Forgotten)
- **Data Retention Policy**: Dữ liệu cá nhân chỉ được lưu trữ trong thời gian khách hàng sử dụng dịch vụ hoặc theo thời hạn pháp luật quy định. Hết thời hạn, hệ thống tự động chạy job xóa sạch (Purge).
- **Quyền Xóa & Rút lại đồng ý (Right to Erasure & Consent Revocation)**: Xây dựng API chuẩn cho phép người dùng gửi yêu cầu xóa dữ liệu cá nhân. Khi nhận yêu cầu, hệ thống phải thực hiện xóa hoàn toàn thông tin cá nhân khỏi Database chính và bản backup trong vòng **72 giờ**, đồng thời lưu vết vào nhật ký tuân thủ.
- **Hồ sơ DPIA (Data Protection Impact Assessment)**: BA và Compliance Officer bắt buộc lập Hồ sơ Đánh giá tác động xử lý dữ liệu cá nhân (Biểu mẫu DPIA) cho mọi tính năng mới có thu thập thông tin người dùng.

---

## 11. BỘ CHỈ SỐ DORA METRICS VÀ ĐO LƯỜNG NĂNG LỰC SẢN XUẤT (AGILE & ENGINEERING DASHBOARD)

### 11.1 Bộ Chỉ Số DORA Metrics Bắt Buộc
Công ty áp dụng chuẩn 4 chỉ số DORA (DevOps Research and Assessment) để đo lường và tối ưu hóa hiệu suất phát triển phần mềm:

| Chỉ số DORA | Ý nghĩa & Cấu trúc đo lường | Mục tiêu chuẩn Công ty (Target) | Cấp độ Đạt được |
| :--- | :--- | :---: | :---: |
| **Deployment Frequency (DF)** | Tần suất triển khai thành công code lên môi trường Production. | **≥ 1 lần / tuần** (Tiến tới daily deploy) | High / Elite Performer |
| **Lead Time for Changes (LTC)** | Thời gian từ khi commit code đầu tiên đến khi code chạy chính thức trên Production. | **< 3 ngày làm việc** | High Performer |
| **Change Failure Rate (CFR)** | Tỷ lệ phần trăm đợt release gây ra sự cố Prod (P1/P2) phải Hotfix hoặc Rollback. | **< 5%** | Elite Performer |
| **Mean Time to Restore (MTTR)** | Thời gian trung bình để khôi phục dịch vụ khi xảy ra sự cố trên Production. | **< 1 giờ** | Elite Performer |

### 11.2 Chỉ Số KPIs Chất Lượng Kỹ Thuật & Hiệu Suất Agile (Engineering Quality KPIs)
Bên cạnh DORA, các nhóm phát triển được đánh giá qua bộ chỉ số kỹ thuật định kỳ trong Sprint:

1. **Sprint Velocity**: Tổng số Story Points hoàn thành đạt DoD trong một Sprint (dùng để tính toán Team Capacity cho Sprint kế tiếp).
2. **Cycle Time**: Thời gian trung bình một Work Item chuyển từ trạng thái `Active` ➔ `Closed`.
3. **Defect Escape Rate**: Tỷ lệ Bug lọt lên Production so với tổng số Bug phát hiện ở giai đoạn Staging/Dev (`Defect Escape Rate = (Prod Bugs / Total Bugs) * 100%`). Mục tiêu: **< 3%**.
4. **Code Coverage Percentage**: Tỷ lệ dòng code được kiểm thử bởi Unit Test tự động. Mục tiêu: **≥ 80%** cho Business Core layer và **≥ 90%** cho Domain Core logic.

### 11.3 Báo Cáo Tự Động Từ Harness Database (Sprint & Release Report Automation)
Loại bỏ hoàn toàn việc làm báo cáo thủ công bằng Excel. Toàn bộ dữ liệu phát triển phần mềm được ghi nhận tự động vào **Harness Database** thông qua CLI và Observability Engine:

- **Báo cáo Sprint Report tự động**: Kết thúc Sprint, Harness CLI tự động truy xuất dữ liệu từ TFS/Jira & Git log để tạo file Markdown Sprint Report chứa: (1) Velocity actual vs plan, (2) Burndown chart data, (3) Danh sách Work Items completed vs carried over, (4) Thống kê Code Coverage.
- **Báo cáo Release Report tự động**: Khi thực hiện Release, Harness CLI tự động quét toàn bộ Git Commits và Pull Requests đã merge để tạo **Release Notes & Audit Log** gửi Stakeholders, bao gồm: Danh sách User Story, Bug fixes, Commits hash, Author, và SHA256 checksum của bản build.

---

## 12. QUẢN TRỊ PHÁT TRIỂN PHẦN MỀM CÓ HỖ TRỢ TRÍ TUỆ NHÂN TẠO (AI-ASSISTED DEVELOPMENT GOVERNANCE & LLM POLICY)

### 12.1 Quy Định Quản Trị Mã Nguồn Do AI Tạo (Human-in-the-Loop & Tagging `[AI]`)
Nhằm khai thác tối đa sức mạnh của AI Assistants (Antigravity, GitHub Copilot, Harness AI) mà vẫn bảo đảm tuyệt đối chất lượng và an toàn mã nguồn:

- **Nguyên tắc Con người Kiểm soát (Human-in-the-Loop)**: AI Agent/Assistant chỉ đóng vai trò hỗ trợ lập trình (Pair Programmer). **100% mã nguồn do AI gợi ý hoặc khởi tạo BẮT BUỘC phải được Lập trình viên (Human Dev) rà soát, kiểm thử và chịu trách nhiệm pháp lý/kỹ thuật cuối cùng**.
- **Quy định Gắn Nhãn `[AI]` (AI Code Tagging)**:
  - Tất cả các Pull Request chứa mã nguồn do AI khởi tạo > 30% nội dung bắt buộc phải gắn tag nhãn `[AI-Assisted]` trong Title hoặc Description.
  - Các hàm hoặc module quan trọng do AI viết phải có comment header ghi rõ: `// Generated with AI assistance - Verified by: <Developer Name>`.

### 12.2 Bảo Mật Thông Tin & Quy Định Dữ Liệu Khi Sử Dụng LLM (Data Privacy & Secret Prohibition)
- **Nghiêm cấm gửi Secret & PII vào LLM**: TUYỆT ĐỐI CẤM người dùng và AI Agent đưa các dữ liệu sau vào prompt gửi lên các hệ thống LLM thương mại công cộng (Public LLM):
  1. Passwords, API Keys, Private Keys, Database Connection Strings.
  2. Dữ liệu cá nhân khách hàng (PII - Họ tên, SĐT, Email, Ngân hàng, CCCD).
  3. Mã nguồn thuật toán độc quyền cốt lõi của Công ty chưa được công khai.
- **Sử dụng Enterprise / Self-Hosted LLM**: Tất cả công cụ AI phục vụ phát triển phần mềm tại Công ty phải sử dụng tài khoản Enterprise (cam kết Zero Data Retention - không dùng dữ liệu khách hàng để train model) hoặc các mô hình Self-Hosted LLM chạy trong hạ tầng VPC nội bộ.

### 12.3 Cổng Phê Duyệt Tự Động & Giới Hạn Quyền AI Agent (Approval Gates & Restricted Permissions)
Định rõ ranh giới quyền hạn tự động của các AI Agent (Harness Agent / Autonomous Coding Subagents):

```text
┌────────────────────────────────────────┐    ┌────────────────────────────────────────┐
│ HÀNH ĐỘNG AI AGENT ĐƯỢC PHÉP TỰ ĐỘNG   │    │ HÀNH ĐỘNG AI AGENT BỊ CẤM HOÀN TOÀN    │
├────────────────────────────────────────┤    ├────────────────────────────────────────┤
│ • Viết Unit Test & Refactor code local │    │ ❌ Tự động Merge PR vào branch `main`  │
│ • Tạo Pull Request (Draft PR)          │    │ ❌ Tự động Deploy lên Production       │
│ • Phân tích Log lỗi & Gợi ý bản vá     │    │ ❌ Tự động Thay đổi Schema DB Prod     │
│ • Sinh tài liệu API Docs / Draft Spec  │    │ ❌ Tự động Xóa dữ liệu hoặc Repositories│
└────────────────────────────────────────┘    └────────────────────────────────────────┘
```

- **Bắt buộc Human Approval Gate**: Mọi thao tác Merge code vào branch chính (`main`), Phát hành Release, hoặc Thay đổi cấu hình Production của AI Agent **bắt buộc phải đi qua cổng duyệt thủ công (Human Approval Gate)** của Tech Lead hoặc DevOps.

### 12.4 Audit Logging & Observability Cho AI Agent (Harness Observability Audit Trail)
- **Ghi vết 100% thao tác của Agent**: Toàn bộ hoạt động của AI Agent (Prompt, Tool Call, Executed Commands, Created/Modified Files, Git Commits) phải được tự động ghi vết và lưu trữ tập trung trên **Harness Observability DB**.
- **Truy xuất Nguồn gốc (Traceability)**: Nhật ký AI Audit Log phải được bảo tồn tối thiểu **3 năm** để phục vụ công tác rà soát an ninh mạng, kiểm toán tuân thủ ISO/IEC 27001:2022 và đánh giá hiệu quả ứng dụng AI trong doanh nghiệp.
