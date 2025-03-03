#services.py
class PaymentStep:
    def execute(self, context):
        print("Executing Payment Step...")
        #Simulate payment processing
        context["payment_id"] = "PAY12345"
        #Simulate a failure (uncomment to test failure)
        #raise Exception("Payment failed: Insufficient funds")
        print("Payment Step completed successfully!")

    def compensate(self, context):
        print("Compensating Payment Step...")
        payment_id = context.get("payment_id")
        if payment_id:
            print(f"Refunding payment {payment_id}...")
            print("Payment compensation completed!")


class InventoryStep:
    def execute(self, context):
        print("Executing Inventory Step...")
        #Simulate inventory reservation
        context["inventory_reservation_id"] = "INV67890"
        #Simulate a failure (uncomment to test failure)
        #raise Exception("Inventory failed: Out of stock")
        print("Inventory Step completed successfully!")

    def compensate(self, context):
        print("Compensating Inventory Step...")
        reservation_id = context.get("inventory_reservation_id")
        if reservation_id:
            print(f"Releasing inventory reservation {reservation_id}...")
            print("Inventory compensation completed!")


class ShippingStep:
    def execute(self, context):
        print("Executing Shipping Step...")
        #Simulate shipping creation
        context["shipping_id"] = "SHIP54321"
        #Simulate a failure (uncomment to test failure)
        #raise Exception("Shipping failed: Address invalid")
        print("Shipping Step completed successfully!")

    def compensate(self, context):
        print("Compensating Shipping Step...")
        shipping_id = context.get("shipping_id")
        if shipping_id:
            print(f"Cancelling shipping {shipping_id}...")
            print("Shipping compensation completed!")

"""class PaymentStep():
    def execute(self, context):
    print("Executing payment step...")
    """