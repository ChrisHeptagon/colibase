"use client"
import { useState } from "react"

type FormType = "login" | "register"

export function LogInItForm({
  title
 }: {
    title: string
 }) {
  const [FormType, setFormType] = useState<FormType>("register")
  let type: FormType = "register"
  if (title === "Login") {
    type = "login"
  } else if (title === "Register") {
    type = "register"
  }
  return (
    <>
      <h1>
        {title}
      </h1>
      <form>
        <input type="text" placeholder="Username" />
        <input type="password" placeholder="Password" />
        <button type="submit">
          {title}
        </button>
      </form>
    </>
  )
}