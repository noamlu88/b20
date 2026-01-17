from flask import Blueprint
from controllers.transaction import get_transactions, create_transaction, update_transaction, delete_transaction

transaction_bp = Blueprint('transaction', __name__)

transaction_bp.route('/transactions', methods=['GET'])(get_transactions)
transaction_bp.route('/transactions', methods=['POST'])(create_transaction)
transaction_bp.route('/transactions/<int:id>', methods=['PUT'])(update_transaction)
transaction_bp.route('/transactions/<int:id>', methods=['DELETE'])(delete_transaction)
