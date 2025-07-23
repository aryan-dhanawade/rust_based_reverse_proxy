from flask import Flask, request

app = Flask(__name__)

@app.route('/')
def hello_world():
    return "what it is"


@app.route('/health')
def health_check():
    return 'OK', 200

if __name__ == '__main__':
    app.run(debug=True, port=8081)