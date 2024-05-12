# Route-Rally-API
API that uses google maps distance matrix and randomixation to determine an optimized travel route taking a google maps link as an input. Also uses the 'Basic Auth' library for authentication.

For compiling and running yourself you will need to follow the setup in https://docs.rs/google_maps/3.5.0/google_maps/distance_matrix/index.html and get an API key for Google Distance Matrix. As of right now the API Keys are pay as you go and free as long as you stay below a certain limit.

Before using the api be sure to set an admin token.

A general command may look like this:

curl -X POST http://localhost:3698/api/v1/get-route -H "Content-Type: application/json" -d '{"url": "google-map-url-here", "token":"enter_token-here"}'
