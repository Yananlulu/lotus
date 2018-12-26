import {SIGN_IN, SIGN_OUT} from '../actions'

const currentUser = (state = {}, action) => {
  switch (action.type) {
    case SIGN_IN:
      // TODO
      return {uid: 'aaa'}
    case SIGN_OUT:
      // TODO
      return {}
    default:
      return state
  }
}

export default currentUser
