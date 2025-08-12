export async function fetchNotifications(token) {
  const res = await fetch('http://localhost:8000/notifications', {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!res.ok) {
    throw new Error('Failed to fetch notifications');
  }

  return res.json();
}
