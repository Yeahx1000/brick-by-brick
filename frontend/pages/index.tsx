import HabitList from '../components/HabitList';
import HabitForm from '../components/HabitForm';
import NotificationList from '../components/NotificationList';

const HomePage = () => {
  return (
    <div>
      <h1>Welcome to Brick by Brick</h1>
      <HabitForm />
      <HabitList />
      <NotificationList />
    </div>
  );
};

export default HomePage;
