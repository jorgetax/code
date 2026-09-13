export default function Layout(content, title = 'Page') {
  document.title = title
  return `<main>${content}</main>`
}