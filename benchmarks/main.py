from flask import Flask, send_from_directory

app = Flask(__name__)

@app.route('/')
def serve_homepage():
    # Replace the path with the actual path to your HTML file
    return send_from_directory('/Users/mmuhammad/Desktop/projects/nashar_gah/assets', 'index.html')

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=8080)
