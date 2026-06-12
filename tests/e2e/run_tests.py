#!/usr/bin/env python3
import sys
import unittest
import os

def main():
    # Make sure we are in the correct directory context
    script_dir = os.path.dirname(os.path.abspath(__file__))
    sys.path.insert(0, script_dir)
    
    print("======================================================================")
    print("Starting E2E Test Suite Execution (Dual Track - Testing Track)")
    print("======================================================================")
    
    # Load suite from test_suite
    import test_suite
    loader = unittest.TestLoader()
    suite = loader.loadTestsFromModule(test_suite)
    
    # Run tests
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    print("\n======================================================================")
    print("Test Suite Summary")
    print("======================================================================")
    print(f"Total Tests Run: {result.testsRun}")
    print(f"Passed: {result.testsRun - len(result.failures) - len(result.errors)}")
    print(f"Failures: {len(result.failures)}")
    print(f"Errors: {len(result.errors)}")
    print("======================================================================")
    
    if not result.wasSuccessful():
        print("Outcome: FAILED", file=sys.stderr)
        sys.exit(1)
    else:
        print("Outcome: PASSED")
        sys.exit(0)

if __name__ == "__main__":
    main()
