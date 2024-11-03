import { useEffect, useState } from 'react';
import axios from 'axios';
import HabitItem from './HabitItem';

export interface Habit {
  id: string;
  name: string;
  description: string;
  completed: boolean;
  created_at: string;
}

const HabitList = () => {
  const [habits, setHabits] = useState<Habit[]>([]);

  useEffect(() => {
    const fetchHabits = async () => {
      try {
        const response = await axios.get('http://127.0.0.1:8080/habits');
        setHabits(response.data);
      } catch (error) {
        console.error('Error fetching habits:', error);
      }
    };

    fetchHabits();
  }, []);

  return (
    <div>
      <h2>Your Habits</h2>
      <ul>
        {habits.map(habit => (
          <HabitItem key={habit.id} habit={habit} />
        ))}
      </ul>
    </div>
  );
};

export default HabitList;
