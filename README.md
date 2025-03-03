# SystemDesign
To implement the Saga Pattern for the e-commerce checkout workflow, we design a system that orchestrates three steps: Payment, Inventory, and Shipping. Each step has "do" and "compensate" actions. If any step fails, the Saga orchestrator triggers compensation in reverse order.

#Design Overview
Saga Orchestrator: Manages the execution of steps and handles compensation if any step fails.
Steps (Payment, Inventory, Shipping): Each step is a class with execute() (do action) and compensate() (compensation action) methods.
Context: A shared dictionary passed between steps to store transaction data (e.g., payment ID).

#Code Structure
services.py: Contains the PaymentStep, InventoryStep, and ShippingStep classes.
saga.py: Implements the SagaOrchestrator class.
main.py: Demonstrates the workflow execution.

#How It Works
- Execution:
The SagaOrchestrator executes each step's execute() method in sequence.
If all steps succeed, the workflow completes successfully.
- Compensation:
If any step fails, the orchestrator triggers the compensate() method for all completed steps in reverse order.
- Context:
A shared context dictionary is used to pass data (e.g., payment ID, reservation ID) between steps.
