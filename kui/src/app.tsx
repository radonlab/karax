import { createRoot } from 'react-dom/client';

const App = () => {
  return <h3>Hello</h3>;
};

const root = createRoot(document.getElementById('root'));
root.render(<App />);
