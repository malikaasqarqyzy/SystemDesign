#saga.py
class SagaError(Exception):
    """Custom exception for Saga failures."""
    pass


class SagaOrchestrator:
    def __init__(self, steps):
        self.steps = steps
        self.context = {}

    def execute(self):
        completed_steps = []
        try:
            for step in self.steps:
                step.execute(self.context)
                completed_steps.append(step)
            print("All steps completed successfully!")
        except Exception as e:
            print(f"Step failed: {e}")
            self._compensate(completed_steps)
            raise SagaError("Saga execution failed. Compensation completed.")

    def _compensate(self, completed_steps):
        print("Starting compensation...")
        for step in reversed(completed_steps):
            try:
                step.compensate(self.context)
            except Exception as e:
                print(f"Compensation failed for step: {e}")
        print("Compensation process completed.")