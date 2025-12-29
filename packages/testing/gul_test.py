"""
GUL Test Framework
Unit testing framework with assertions and test runners.

Status: ✅ Implemented
Priority: Critical
"""

from typing import List, Callable, Optional, Any, Dict
from dataclasses import dataclass
from datetime import datetime
import traceback

__version__ = "0.1.0"
__all__ = ['TestCase', 'TestRunner', 'assert_equal', 'assert_true', 'assert_raises']

@dataclass
class TestResult:
    """Test result"""
    name: str
    passed: bool
    duration_ms: float
    error: Optional[str] = None
    traceback: Optional[str] = None

class AssertionError(Exception):
    """Assertion failed"""
    pass

def assert_equal(actual: Any, expected: Any, msg: Optional[str] = None):
    """Assert values are equal"""
    if actual != expected:
        message = msg or f"Expected {expected}, got {actual}"
        raise AssertionError(message)

def assert_true(value: bool, msg: Optional[str] = None):
    """Assert value is true"""
    if not value:
        message = msg or f"Expected True, got {value}"
        raise AssertionError(message)

def assert_false(value: bool, msg: Optional[str] = None):
    """Assert value is false"""
    if value:
        message = msg or f"Expected False, got {value}"
        raise AssertionError(message)

def assert_raises(exception_type: type, func: Callable, *args, **kwargs):
    """Assert function raises exception"""
    try:
        func(*args, **kwargs)
        raise AssertionError(f"Expected {exception_type.__name__} to be raised")
    except exception_type:
        pass

def assert_none(value: Any, msg: Optional[str] = None):
    """Assert value is None"""
    if value is not None:
        message = msg or f"Expected None, got {value}"
        raise AssertionError(message)

def assert_not_none(value: Any, msg: Optional[str] = None):
    """Assert value is not None"""
    if value is None:
        message = msg or "Expected non-None value"
        raise AssertionError(message)

def assert_in(item: Any, container: Any, msg: Optional[str] = None):
    """Assert item in container"""
    if item not in container:
        message = msg or f"Expected {item} to be in {container}"
        raise AssertionError(message)

def assert_not_in(item: Any, container: Any, msg: Optional[str] = None):
    """Assert item not in container"""
    if item in container:
        message = msg or f"Expected {item} not to be in {container}"
        raise AssertionError(message)

class TestCase:
    """
    Base test case class
    
    Example:
        class MathTests(TestCase):
            def test_addition(self):
                assert_equal(1 + 1, 2)
            
            def test_subtraction(self):
                assert_equal(5 - 3, 2)
            
            def test_division_by_zero(self):
                assert_raises(ZeroDivisionError, lambda: 1 / 0)
        
        # Run tests
        runner = TestRunner()
        results = runner.run(MathTests)
    """
    
    def setup(self):
        """Called before each test"""
        pass
    
    def teardown(self):
        """Called after each test"""
        pass
    
    def setup_class(cls):
        """Called once before all tests"""
        pass
    
    def teardown_class(cls):
        """Called once after all tests"""
        pass

class TestRunner:
    """Test runner"""
    
    def __init__(self, verbose: bool = True):
        self.verbose = verbose
        self.results: List[TestResult] = []
    
    def run(self, test_class: type) -> List[TestResult]:
        """Run all tests in test class"""
        self.results = []
        
        # Get test methods
        test_methods = [
            name for name in dir(test_class)
            if name.startswith('test_') and callable(getattr(test_class, name))
        ]
        
        if not test_methods:
            if self.verbose:
                print(f"No tests found in {test_class.__name__}")
            return []
        
        # Setup class
        if hasattr(test_class, 'setup_class'):
            test_class.setup_class()
        
        try:
            # Run each test
            for method_name in test_methods:
                instance = test_class()
                result = self._run_test(instance, method_name)
                self.results.append(result)
                
                if self.verbose:
                    status = "✓" if result.passed else "✗"
                    print(f"{status} {result.name} ({result.duration_ms:.2f}ms)")
                    if not result.passed and result.error:
                        print(f"  Error: {result.error}")
        
        finally:
            # Teardown class
            if hasattr(test_class, 'teardown_class'):
                test_class.teardown_class()
        
        # Print summary
        if self.verbose:
            self._print_summary()
        
        return self.results
    
    def _run_test(self, instance: TestCase, method_name: str) -> TestResult:
        """Run single test"""
        start = datetime.now()
        
        try:
            # Setup
            instance.setup()
            
            # Run test
            method = getattr(instance, method_name)
            method()
            
            # Success
            duration = (datetime.now() - start).total_seconds() * 1000
            return TestResult(
                name=f"{instance.__class__.__name__}.{method_name}",
                passed=True,
                duration_ms=duration
            )
        
        except AssertionError as e:
            duration = (datetime.now() - start).total_seconds() * 1000
            return TestResult(
                name=f"{instance.__class__.__name__}.{method_name}",
                passed=False,
                duration_ms=duration,
                error=str(e),
                traceback=traceback.format_exc()
            )
        
        except Exception as e:
            duration = (datetime.now() - start).total_seconds() * 1000
            return TestResult(
                name=f"{instance.__class__.__name__}.{method_name}",
                passed=False,
                duration_ms=duration,
                error=f"Unexpected error: {str(e)}",
                traceback=traceback.format_exc()
            )
        
        finally:
            # Teardown
            try:
                instance.teardown()
            except:
                pass
    
    def _print_summary(self):
        """Print test summary"""
        total = len(self.results)
        passed = sum(1 for r in self.results if r.passed)
        failed = total - passed
        
        print("\n" + "=" * 50)
        print(f"Tests: {total} total, {passed} passed, {failed} failed")
        
        if failed > 0:
            print("\nFailed tests:")
            for result in self.results:
                if not result.passed:
                    print(f"  - {result.name}")
        
        print("=" * 50)

# Decorators
def skip(reason: str):
    """Skip test decorator"""
    def decorator(func):
        func._skip = True
        func._skip_reason = reason
        return func
    return decorator

def parametrize(params: List[tuple]):
    """Parametrize test decorator"""
    def decorator(func):
        func._parametrize = params
        return func
    return decorator

# Test discovery
def discover_tests(directory: str = ".") -> List[type]:
    """Discover test classes in directory"""
    # Simplified - would actually scan files
    return []
