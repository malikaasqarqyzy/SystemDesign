import random

class PaymentService:
    def do(self):
        print("Processing payment...")
        if random.random() < 0.3:
            raise Exception("Payment failed")
    
    def compensate(self):
        print("Compensating payment: refunding...")

class InventoryService:
    def do(self):
        print("Updating inventory...")
        if random.random() < 0.3:
            raise Exception("Inventory update failed")
    
    def compensate(self):
        print("Compensating inventory: restocking...")

class ShippingService:
    def do(self):
        print("Arranging shipping...")
        if random.random() < 0.3:
            raise Exception("Shipping failed")
    
    def compensate(self):
        print("Compensating shipping: canceling shipment...")

class SagaOrchestrator:
    def __init__(self, steps):
        self.steps = steps
        self.executed_steps = []
    
    def execute(self):
        try:
            for step in self.steps:
                step.do()
                self.executed_steps.append(step)
            return True
        except Exception as e:
            print(f"\nError: {e}")
            self.compensate()
            return False
    
    def compensate(self):
        print("\n--- Starting compensation ---")
        for step in reversed(self.executed_steps):
            step.compensate()
        print("--- Compensation completed ---\n")

def main():
    steps = [PaymentService(), InventoryService(), ShippingService()]
    orchestrator = SagaOrchestrator(steps)
    success = orchestrator.execute()
    if success:
        print("\nCheckout completed successfully!")
    else:
        print("\nCheckout failed. All compensations applied.")

if __name__ == "__main__":
    main()