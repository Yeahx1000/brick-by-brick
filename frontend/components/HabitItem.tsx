// frontend/src/components/HabitItem.tsx

import { Habit } from './HabitList';

interface HabitItemProps {
  habit: Habit;
}

const HabitItem: React.FC<HabitItemProps> = ({ habit }) => {
  return (
    <li>
      <strong>{habit.name}</strong>: {habit.description} - {habit.completed ? "✅" : "❌"}
    </li>
  );
};

export default HabitItem;
