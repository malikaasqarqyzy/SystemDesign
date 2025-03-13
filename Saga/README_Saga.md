# SystemDesign
To implement the Saga Pattern for the e-commerce checkout workflow, we design a system that orchestrates three steps: Payment, Inventory, and Shipping. Each step has "do" and "compensate" actions. If any step fails, the Saga orchestrator triggers compensation in reverse order.

#Design Overview

Saga Orchestrator: Manages the execution of steps and handles compensation if any step fails.
Steps (Payment, Inventory, Shipping): Each step is a class with execute() (do action) and compensate() (compensation action) methods.
Context: A shared dictionary passed between steps to store transaction data (e.g., payment ID).

#Code Structure

- services.py: Contains the PaymentStep, InventoryStep, and ShippingStep classes.
- saga.py: Implements the SagaOrchestrator class.
- main.py: Demonstrates the workflow execution.

#How It Works
- Execution:
The SagaOrchestrator executes each step's execute() method in sequence.
If all steps succeed, the workflow completes successfully.
- Compensation:
If any step fails, the orchestrator triggers the compensate() method for all completed steps in reverse order.
- Context:
A shared context dictionary is used to pass data (e.g., payment ID, reservation ID) between steps.

#Design Overview
This implementation models a Saga Pattern within a single microservice to manage the e-commerce checkout process. The workflow includes three sequential steps: Payment, Inventory Update, and Shipping. Each step supports a do action (to perform the operation) and a compensate action (to revert it if a subsequent step fails).

#Key Components
- Services (PaymentService, InventoryService, ShippingService):
- Each service implements do() to execute the operation and compensate() to undo it.
- Random failures are simulated (30% chance) to demonstrate error handling.

- Saga Orchestrator:
- Manages the workflow sequence.
- Tracks completed steps in executed_steps.
- If any step fails, triggers compensation in reverse order of completion.

#Workflow Logic
Execution:
- Steps run sequentially: Payment → Inventory → Shipping.
- Successful steps are logged in executed_steps.

Compensation:
- If a step fails, the orchestrator runs compensate() on all completed steps in reverse order.
Example: If Shipping fails, Inventory and Payment are compensated in that order.
