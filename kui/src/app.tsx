import { createRoot } from 'react-dom/client';

const App = () => {
  return <h3>Hello</h3>;
};

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(<App />);
}
