import { fetcher, User } from "@/types"
import useSWR from "swr"

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export default function LoginView({ user_id }: { user_id: number }) {

    const { data, error, isLoading } = useSWR<User>('/api/user', fetcher);

    if (isLoading) {
        return <span></span>
    } else if (error) {
        return <span>{"Could not find user"}</span>
    } else {
        return <span>{data?.id} +++++ {data?.name} </span>
    }

}