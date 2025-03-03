#main.py
from saga import SagaOrchestrator, SagaError
from services import PaymentStep, InventoryStep, ShippingStep

def main():
    #Define the steps for the checkout workflow
    steps = [PaymentStep(), InventoryStep(), ShippingStep()]
    orchestrator = SagaOrchestrator(steps)

    try:
        #Execute the saga
        orchestrator.execute()
        print("Checkout completed successfully!")
    except SagaError as e:
        print(f"Checkout failed: {e}")

if __name__ == "__main__":
    main()