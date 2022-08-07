export interface Countdown {
    id: string,
    name: string,
    date: string
}

/** @type {import('./__types/[id]').RequestHandler} */
export async function GET({ params }: { params: any }) {
    // `params.id` comes from [id].js
    const id = params?.id

    if (!id) {
        return {
            status: 404
        }
    }

    const data = await fetch(`${import.meta.env.VITE_BACKEND_URL}/event/${id}`)
        .then((response) => response.json())
        .catch((err) => {
            console.error(err)
            return {
                status: 404
            }
        })

    if (data) {
        const countdown: Countdown = data
        return {
            status: 200,
            headers: {},
            body: { countdown }
        };
    }

    return {
        status: 404
    };
}