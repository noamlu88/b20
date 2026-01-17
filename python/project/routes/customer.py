from flask import Blueprint
from controllers.customer import get_customers, create_customer, update_customer, delete_customer

customer_bp = Blueprint('customer', __name__)

customer_bp.route('/customers', methods=['GET'])(get_customers)
customer_bp.route('/customers', methods=['POST'])(create_customer)
customer_bp.route('/customers/<int:id>', methods=['PUT'])(update_customer)
customer_bp.route('/customers/<int:id>', methods=['DELETE'])(delete_customer)
