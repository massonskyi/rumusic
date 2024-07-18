import os
import subprocess
import shlex
from tqdm import tqdm


def run_command(command):
    process = subprocess.Popen(shlex.split(command), stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    output, error = process.communicate()
    if error:
        print(f"Error: {error.decode('utf-8')}")
    return output.decode("utf-8")


# Replace these variables with your own values
db_user = "user"
db_password = "1111"
db_name = "musicdb"
dump_files_path = "./"
container_name = "music_player_db"

# Start a PostgreSQL container with a data container
print("Starting PostgreSQL container...")
run_command(
    f"docker run -d --name {container_name} -e POSTGRES_USER={db_user} -e POSTGRES_PASSWORD={db_password} -e POSTGRES_DB={db_name} --volumes-from {container_name} -p 5432:5432 postgres:latest")

# Wait for the PostgreSQL server to start
print("Waiting for PostgreSQL server to start...")
run_command("sleep 10")

# List all SQL dump files in the given directory
print("Found the following SQL dump files:")
dump_files = os.listdir(dump_files_path)
dump_files = [f for f in dump_files if f.endswith(".sql")]
for dump_file in dump_files:
    print(f"- {dump_file}")

# Import each SQL dump file into the database with a progress bar
print("\nImporting SQL dumps:")
for dump_file in tqdm(dump_files, desc="Progress", leave=False):
    dump_file_path = os.path.join(dump_files_path, dump_file)
    run_command(f"docker exec -i {container_name} psql -U {db_user} -d {db_name} -f {dump_file_path}")

print("\nDatabase creation and configuration completed.")
