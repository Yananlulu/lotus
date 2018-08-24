import {notification} from 'antd'
import moment from 'moment'

export const failed = (err) => notification.error({message: moment().format('ll LTS'), description: JSON.stringify(err), duration: 30})

const status = (res) => {
  if (res.status === 200) {
    return Promise.resolve(res)
  }
  return Promise.reject(res.statusText)
}

const json = (res) => res.json()

const api = (url) => `/api${url}`

export const get = (url) => fetch(api(url)).then(status).then(json)

export const delete_ = (url) => fetch(api(url)).then(status).then(json)
