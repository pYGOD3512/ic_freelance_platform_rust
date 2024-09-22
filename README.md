# Decentralized Freelance Platform

A smart contract-based freelancing system on the Internet Computer.

## About

This project implements a basic freelance platform using the Internet Computer Protocol. It allows users to register, post jobs, and view job listings.

## Setup

1. Clone the repository
2. Install dfx
3. Run `dfx start --background`
4. Deploy with `dfx deploy`

## Usage

Here are some basic commands to interact with the canister:

Register a user:
```
dfx canister call freelance_platform register_user '("user_id")'
```

Post a job:
```
dfx canister call freelance_platform post_job '("client_id", "Job Title", "Description", 1000)'
```

Get job details:
```
dfx canister call freelance_platform get_job '("job_id")'
```

List open jobs:
```
dfx canister call freelance_platform list_open_jobs
```

## Project Structure

- `src/freelance_platform/`: Canister code
- `src/freelance_platform/freelance_platform.did`: Candid interface
- `dfx.json`: Project configuration

## Development

To make changes:
1. Edit the code in `src/freelance_platform/src/lib.rs`
2. Rebuild and redeploy:
   ```
   dfx build
   dfx canister install freelance_platform --mode upgrade
   ```

## Future Work

- Implement bidding system
- Add reputation tracking
- Develop payment integration

## License

This project is under the MIT License.
