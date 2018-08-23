import {
  GraphQLClient
} from 'graphql-request'
import {
  notification
} from 'antd'
import moment from 'moment'

import {
  get as getToken
} from './token'

export const client = () => {
  return new GraphQLClient('/graphql', {
    headers: {
      Authorization: `Bearer ${getToken()}`
    },
    credentials: 'include',
    mode: 'cors'
  })
}

// -----------------------------------------------------------------------------

export const failed = (err) => notification.error({
  message: moment().format('ll LTS'),
  description: JSON.stringify(err),
  duration: 30
})

const status = (res) => {
  if (res.status === 200) {
    return Promise.resolve(res)
  }
  return Promise.reject(res.statusText)
}

const json = (res) => res.json()

const api = (url) => `/api${url}`

export const get = (url, data) => fetch(api(url))
  .then(status)
  .then(json)

export const delete_ = (url, data) => fetch(api(url))
  .then(status)
  .then(json)