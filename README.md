# mlflow-tenant-proxy


## Goals
1. Isolate different tenants from each other.
2. Authentication is header driven
3. Authorization is handled outside of the scope of the server (e.g. use OPA)