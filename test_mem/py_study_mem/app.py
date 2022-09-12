from flask import Flask
from flask import jsonify
import listener

app = Flask(__name__)

map_services = {}

@app.route("/")
def hello_world():
    return "<p>Hello, World! MyApp</p>"

@app.route("/listen/<pid>")
def listen(pid):
    service = listener.start(pid)
    map_services[pid] = service
    return f"<p>Listening process... {pid} [OK]</p>"


@app.route("/stop/<pid>")
def stop(pid):
    if pid in map_services:
        #service = map_services[pid]
        service = map_services.pop(pid)
        listener.stop(service)
        return f"<p>Listening process... {pid} [STOP]</p>"
    else: 
        return f"<p>Dont found process... {pid}</p>"

@app.route("/report/<pid>")
def report(pid):
    if pid in map_services:
        service = map_services[pid]
        data = service.get_report()
        response = {
            'report': 'success',
            'info': data,
        }
        return jsonify(response)
    else:
        response = {'report': 'error'}
        return jsonify(response)

if __name__ == '__main__':
    app.run(debug=True)