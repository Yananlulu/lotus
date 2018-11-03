import {notification} from 'antd'
import moment from 'moment'

import {get as getToken} from './token'

export const failed = (err) => notification.error({message: moment().format('ll LTS'), description: err, duration: 30})

export const backend = (url) => `/api${url}`

export const options = (method) => {
  return {
    method: method,
    // mode: 'cors',
    credentials: 'include',
    headers: {
      'Authorization': `BEARER ${getToken()}`
    }
  }
}

const parse = (res) => {
  // res.status === 200 || res.status === 0
  return res.ok
    ? res.json()
    : res.text().then(err => {
      throw err
    })
}

export const get = (path) => {
  return fetch(backend(path), options('GET')).then(parse)
}

export const delete_ = (path) => {
  return fetch(backend(path), options('DELETE')).then(parse)
}

// https://github.github.io/fetch/#options
export const post = (path, body) => {
  var data = options('POST')
  data.body = JSON.stringify(body)
  data.headers['Content-Type'] = "application/json; charset=utf-8"
  return fetch(backend(path), data).then(parse)
}

export const patch = (path, body) => {
  var data = options('PATCH')
  data.body = JSON.stringify(body)
  return fetch(backend(path), data).then(parse)
}

export const put = (path, body) => {
  var data = options('PUT')
  data.body = JSON.stringify(body)
  return fetch(backend(path), data).then(parse)
}
