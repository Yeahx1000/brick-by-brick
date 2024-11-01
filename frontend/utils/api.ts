import axios from 'axios';

export interface Habit {
  id: string; 
  name: string;
  description: string;
  completed: boolean;
  created_at: string; // can also use Date object
}

const apiClient = axios.create({
  baseURL: 'http://127.0.0.1:8080', // Base URL for the API
});


export const getHabits = async (): Promise<Habit[]> => {
  const response = await apiClient.get('/habits');
  return response.data;
};

export const createHabit = async (habit: Habit): Promise<Habit> => {
  const response = await apiClient.post('/habits', habit);
  return response.data;
};
