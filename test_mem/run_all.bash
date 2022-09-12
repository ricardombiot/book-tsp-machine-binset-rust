#!bin/bash
echo "NOTE: Remeber run flask server... with:"
echo "    sh ./py_study_mem/start_server.sh"
echo "and compile release rust with:"
echo "    cargo build --release"
echo "--------------------------------------"
for i in {4..12}
do
  bash ./run.bash $i
done
