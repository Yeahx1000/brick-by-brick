import axios from 'axios';

/* local development URL below, if needed for testing uncomment code below and 
change axios.get and axios.post to only API_URL, not process.env.API_URL 
in production do not expose keys and .env variables */ 

// const API_URL = 'http://localhost:8080/habits';

export const getHabits = async () => {
    const response = await axios.get(process.env.API_URL);
    return response.data;
};

export const createHabit = async (habit) => {
    const response = await axios.post(process.env.API_URL, habit);
    return response.data;
};

// Add other functions for update and delete as needed
