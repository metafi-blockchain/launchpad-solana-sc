import { Route, Routes } from "react-router-dom";
import './App.css';
import NotFound from "./modules/not-found";
import Homepage from "./modules/homepage";
import ScrollToTop from "./components/scrool-to-top";
import {Buffer} from 'buffer';
window.Buffer = Buffer;

const App = () => {
  return (
    <ScrollToTop>
      <Routes>
        <Route path='*' element={<NotFound />} />
        <Route path='/' element={<Homepage />} />
      </Routes>
    </ScrollToTop>
  );
};

export default App;