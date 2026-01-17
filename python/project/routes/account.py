from flask import Blueprint
from controllers.account import get_accounts, create_account, update_account, delete_account

account_bp = Blueprint('account', __name__)

account_bp.route('/accounts', methods=['GET'])(get_accounts)
account_bp.route('/accounts', methods=['POST'])(create_account)
account_bp.route('/accounts/<int:id>', methods=['PUT'])(update_account)
account_bp.route('/accounts/<int:id>', methods=['DELETE'])(delete_account)
