import PageNotFoud from "./not-found.js";

const routes = {
  "/404": PageNotFoud
}

export function router() {
  const path = window.location.pathname
  const root = document.getElementById('root')

  root.innerHTML = '';

  const page = routes[path]

  if (page) root.innerHTML = page()
  else {
    window.history.replaceState({}, '', '/404')
    root.innerHTML = PageNotFoud()
  }
}

export function navigate(path) {
  window.history.pushState({}, '', path)
  router()
}

window.addEventListener('popstate', router)
window.addEventListener('DOMContentLoaded', router)
