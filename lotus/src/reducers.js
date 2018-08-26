import {combineReducers} from 'redux'

import {USER_SIGN_IN, USER_SIGN_OUT} from './actions'

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case USER_SIGN_IN:
      console.log(action.token) // TODO
      return {uid: 'chang-me'}
    case USER_SIGN_OUT:
      return {}
    default:
      return state
  }
}

export default combineReducers({currentUser})
