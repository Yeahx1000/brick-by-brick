# Brick by Brick Habit Tracker

### Project still in development.

## Overview

'Brick by Brick' is a habit tracking application built with a Rust backend and a Next.js frontend. The application allows users to add, view, and manage their habits effectively.

Why the name 'Brick by Brick'? Because building healthy habits is like laying bricks. Each brick represents a step towards long lasting healthy habits. (I also just like branding my work)

## Project Structure

- **backend/**: Rust backend directory
- **frontend/**: Next.js frontend directory

## Getting Started

### Prerequisites

- Rust and Cargo
- Node.js and npm

### Setup Backend

1. Navigate to the backend directory:

   ```bash
   cd backend
   ```

2. Install dependencies and run the server:

   ```bash
   cargo run
   ```

### Setup Frontend

1. Navigate to the frontend directory:

   ```bash
   cd frontend
   ```

2. Install dependencies:

   ```bash
   npm install
   ```

3. Start the Next.js development server:

   ```bash
   npm run dev
   ```

### API Endpoints

- **GET /**: Retrieve all habits
- **POST /habits**: Create a new habit

## Usage

1. Open your browser and go to [http://localhost:3000](http://localhost:3000).
2. Add new habits using the form.
3. View your habits listed below the form.
