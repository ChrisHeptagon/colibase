
export async function loginSchema() {
    const schema = await fetch(`http://0.0.0.0:3006/api/login_schema`)
    const json = await schema.json()
    return json
}