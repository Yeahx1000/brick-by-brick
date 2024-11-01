import axios from 'axios';

const API_URL = 'http://localhost:8080/habits';

export const getHabits = async () => {
    const response = await axios.get(API_URL);
    return response.data;
};

export const createHabit = async (habit) => {
    const response = await axios.post(API_URL, habit);
    return response.data;
};

// Add other functions for update and delete as needed
