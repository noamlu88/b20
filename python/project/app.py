import os
from flask import Flask
from dotenv import load_dotenv
from db import close_connection
from routes.customer import customer_bp
from routes.account import account_bp
from routes.transaction import transaction_bp

load_dotenv()

app = Flask(__name__)

# Register Blueprints
app.register_blueprint(customer_bp)
app.register_blueprint(account_bp)
app.register_blueprint(transaction_bp)

# Teardown app context to close DB connection
from models.customer import Customer
from models.account import Account
from models.transaction import Transaction

# Teardown app context to close DB connection
app.teardown_appcontext(close_connection)

with app.app_context():
    Customer.create_table()
    Account.create_table()
    Transaction.create_table()

if __name__ == '__main__':
    port = int(os.environ.get('SERVER_PORT', 8080))
    print(f"Starting server on port {port}")
    app.run(host='0.0.0.0', port=port, debug=True)
