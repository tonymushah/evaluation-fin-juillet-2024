/* eslint-disable */
/**
 * This file was generated by 'vite-plugin-kit-routes'
 *
 *      >> DO NOT EDIT THIS FILE MANUALLY <<
 */

/**
 * PAGES
 */
const PAGES = {
  "/": `/`,
  "/admin": `/admin`,
  "/admin/config": `/admin/config`,
  "/admin/etudiant": `/admin/etudiant`,
  "/admin/etudiant/[etu]": (params: { etu: (string | number) }) => {
    return `/admin/etudiant/${params.etu}`
  },
  "/admin/etudiant/[etu]/[semestre]": (params: { etu: (string | number), semestre: (string | number) }) => {
    return `/admin/etudiant/${params.etu}/${params.semestre}`
  },
  "/admin/imports": `/admin/imports`,
  "/admin/note-insert": `/admin/note-insert`,
  "/admin/note-insert/promotion": `/admin/note-insert/promotion`,
  "/admin/semestre": `/admin/semestre`,
  "/admin/semestre/[sem]": (params: { sem: (string | number) }) => {
    return `/admin/semestre/${params.sem}`
  },
  "/admin/login": `/admin/login`,
  "/client": `/client`,
  "/client/ratrapages": `/client/ratrapages`,
  "/client/semestre/[sem]": (params: { sem: (string | number) }) => {
    return `/client/semestre/${params.sem}`
  },
  "/client/login": `/client/login`,
  "/client/logout": `/client/logout`
}

/**
 * SERVERS
 */
const SERVERS = {
  
}

/**
 * ACTIONS
 */
const ACTIONS = {
  
}

/**
 * LINKS
 */
const LINKS = {
  
}

type ParamValue = string | number | undefined

/**
 * Append search params to a string
 */
export const appendSp = (sp?: Record<string, ParamValue | ParamValue[]>, prefix: '?' | '&' = '?') => {
  if (sp === undefined) return ''

  const params = new URLSearchParams()
  const append = (n: string, v: ParamValue) => {
    if (v !== undefined) {
      params.append(n, String(v))
    }
  }

  for (const [name, val] of Object.entries(sp)) {
    if (Array.isArray(val)) {
      for (const v of val) {
        append(name, v)
      }
    } else {
      append(name, val)
    }
  }

  const formatted = params.toString()
  if (formatted) {
    return `${prefix}${formatted}`
  }
  return ''
}

/**
 * get the current search params
 * 
 * Could be use like this:
 * ```
 * route("/cities", { page: 2 }, { ...currentSP() })
 * ```
 */ 
export const currentSp = () => {
  const params = new URLSearchParams(window.location.search)
  const record: Record<string, string> = {}
  for (const [key, value] of params.entries()) {
    record[key] = value
  }
  return record
}

// route function helpers
type NonFunctionKeys<T> = { [K in keyof T]: T[K] extends Function ? never : K }[keyof T]
type FunctionKeys<T> = { [K in keyof T]: T[K] extends Function ? K : never }[keyof T]
type FunctionParams<T> = T extends (...args: infer P) => any ? P : never

const AllObjs = { ...PAGES, ...ACTIONS, ...SERVERS, ...LINKS }
type AllTypes = typeof AllObjs

export type Routes = keyof AllTypes extends `${string}/${infer Route}` ? `/${Route}` : keyof AllTypes
export const routes = [
	...new Set(Object.keys(AllObjs).map((route) => /^\/.*|[^ ]?\/.*$/.exec(route)?.[0] ?? route)),
] as Routes[]

/**
 * To be used like this: 
 * ```ts
 * import { route } from './ROUTES'
 * 
 * route('site_id', { id: 1 })
 * ```
 */
export function route<T extends FunctionKeys<AllTypes>>(key: T, ...params: FunctionParams<AllTypes[T]>): string
export function route<T extends NonFunctionKeys<AllTypes>>(key: T): string
export function route<T extends keyof AllTypes>(key: T, ...params: any[]): string {
  if (AllObjs[key] as any instanceof Function) {
    const element = (AllObjs as any)[key] as (...args: any[]) => string
    return element(...params)
  } else {
    return AllObjs[key] as string
  }
}

/**
* Add this type as a generic of the vite plugin `kitRoutes<KIT_ROUTES>`.
*
* Full example:
* ```ts
* import type { KIT_ROUTES } from './ROUTES'
* import { kitRoutes } from 'vite-plugin-kit-routes'
*
* kitRoutes<KIT_ROUTES>({
*  PAGES: {
*    // here, key of object will be typed!
*  }
* })
* ```
*/
export type KIT_ROUTES = {
  PAGES: { '/': never, '/admin': never, '/admin/config': never, '/admin/etudiant': never, '/admin/etudiant/[etu]': 'etu', '/admin/etudiant/[etu]/[semestre]': 'etu' | 'semestre', '/admin/imports': never, '/admin/note-insert': never, '/admin/note-insert/promotion': never, '/admin/semestre': never, '/admin/semestre/[sem]': 'sem', '/admin/login': never, '/client': never, '/client/ratrapages': never, '/client/semestre/[sem]': 'sem', '/client/login': never, '/client/logout': never }
  SERVERS: Record<string, never>
  ACTIONS: Record<string, never>
  LINKS: Record<string, never>
  Params: { etu: never, semestre: never, sem: never }
}
