import { combineReducers } from "redux";
import applicationReducer from "./applicationReducer";

const rootReducer = combineReducers({
  system: applicationReducer,
});

export default rootReducer;
