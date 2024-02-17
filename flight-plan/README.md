# Install Posgres server  client

Installation of server and client plus the libraries for Diesel installation
```
sudo apt install postgresql postgresql-contrib postgresql-server-dev-13
sudo systemctl status postgres
```

```
sudo su - postgres
$ createuser --pwprompt phil
$ createdb flight_plan
$ createdb -O phil flight_plan
```

# Add diesel crate

```
cargo install diesel_cli --no-default-features --features postgres derive
```