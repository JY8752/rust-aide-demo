// Define an environment named "local"
env "local" {
  // Declare where the schema definition resides.
  // Also supported: ["file://multi.hcl", "file://schema.hcl"].
  src = "file://schema.sql"

  // Define the URL of the database which is managed
  // in this environment.
  url = "postgres://postgres:pass@localhost:5432/app?search_path=public&sslmode=disable"

  // Define the URL of the Dev Database for this environment
  // See: https://atlasgo.io/concepts/dev-database
  dev = "docker://postgres/17/dev?search_path=public"
}