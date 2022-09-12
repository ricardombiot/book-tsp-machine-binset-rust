# Example: sh stop_listen.sh <PID> <name_report>.json
# sh stop_listen.sh 983 instance_5n
curl -o ./reports/$2.json http://localhost:5000/report/$1
curl -v http://localhost:5000/stop/$1