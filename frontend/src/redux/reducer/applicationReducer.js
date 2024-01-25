import { SET_LATEST_BLOCK_NUMBER } from '../types/application';

const initialState = {
    total: 0,
    onDay: 0
};

const applicationReducer = (state = initialState, action) => {
    switch (action.type) {
        case SET_LATEST_BLOCK_NUMBER:
            return {
                ...state,
                ...action.payload
            };
        default:
            return {
                ...state,
            };
    }
};
export default applicationReducer;
