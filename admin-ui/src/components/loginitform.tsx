import { loginSchema } from "@/actions/login_schema"
type FormType = "login" | "register"
import './loginitform.scss'

export async function LogInItForm({
  title
 }: {
    title: string
 }) {
  let type: FormType = "register"
  if (title === "Login") {
    type = "login"
  } else if (title === "Register") {
    type = "register"
  }
  const schema = await loginSchema()
  return (
    <section className="entry_form">
      <h1>
        {title}
      </h1>
      <form method="POST">
         {Object.keys(schema).sort(
            (a, b) => schema[a].order - schema[b].order
          ).map((key) => {
          return (
            <div key={key}>
              <label htmlFor={key}>
                {key}
              </label>
              <input
                type={schema[key].form_type}
                name={key}
                id={key}
                required={schema[key].required}
                pattern={schema[key].pattern}
              />
              </div>
          )
          })}
          <button type="submit">
            {title}
          </button>
      </form>
    </section>
  )
}