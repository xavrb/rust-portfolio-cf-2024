import csv
import random
from datetime import datetime, timedelta

# Open a CSV file in write mode
with open('fake_data.csv', mode='w', newline='') as file:
    writer = csv.writer(file)

    # Write header row
    writer.writerow(['Name', 'Date', 'Amount'])

    # Generate and write 2000 rows of fake data
    for _ in range(2000):
        # Generate fake name
        name = f"User_{random.randint(1, 2000)}"

        # Generate fake date within the last decade
        start_date = datetime.now() - timedelta(days=365 * 10)
        end_date = datetime.now()
        date = start_date + timedelta(seconds=random.randint(0, int((end_date - start_date).total_seconds())))

        # Generate fake amount around 100 USD
        amount = round(random.uniform(90, 110), 2)

        # Write row to CSV file
        writer.writerow([name, date.strftime('%Y-%m-%d'), amount])

