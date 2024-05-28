import { useState } from "react";

export default function useLogin() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const loginUser = async (email: string, password: string) => {
    setIsLoading(true);
    try {
      const response = await fetch("http://localhost:8080/login", {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
      });

      const data = await response.json();
      if (response.status === 200) {
        console.log("Login successful:", data);
        return true;
      } else {
        console.error("Login failed:", data.message);
        setError(data.message);
        return false;
      }
    } catch (error: any) {
      console.error("Error during login:", error);
      setError(error.message);
    } finally {
      setIsLoading(false);
    }
  };

  return { loginUser, isLoading, error };
}
