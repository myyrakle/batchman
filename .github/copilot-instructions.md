# System Architecture

1. API Server: Rust + Axum + SeaORM(SQLite)

- in `/src`

2. Webpage: React + React Router + Typescript

- in `/web`

# API Structure

1. all API paths are in `/src/api.rs`
2. all domains are in `/src/domain`

- all API routes are in `/src/domain/*/routes/http`
- all service(business logic) are in `/src/domain/*/service.rs`. dtos are in `/src/domain/*/dto.rs`, dtos's argument/return type name is `*Request` & `*Response`.
- all repositories are in `/src/domain/*/repository/*.rs`. daos are in `/src/domain/*/dao.rs`, dao's argument type name is `*Params`.
- all entities(DB Table) are in `/src/domain/*/entities.rs`

3. All repositories and services must define an interface through a trait and then implement it.

- The interface trait is in `/src/domain/mod.rs`

# Web Structure

1. Separate page units using react router. (routes are in `/web/src/App.tsx`)
2. Each page is in `/web/src/pages/*.tsx`
3. The individual component units used on each page are in `/web/src/pages/*.tsx`
4. All API call functions are in `/web/src/api.ts`. DO NOT add new file for using server api.

- If necessary, add or modify the API call function by referring to the server code. (`/src/api.rs`, `/src/domain/*/routes/http`, `/src/domain/*/dto.rs`),

# Answer language

- If possible, please answer in Korean.
