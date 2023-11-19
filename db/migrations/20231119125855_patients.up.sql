CREATE TABLE patient (
    id             INTEGER     PRIMARY KEY AUTOINCREMENT,
    patient_id     INTEGER     REFERENCES users(id),
    doctor_id      INTEGER     REFERENCES users(id)
);

CREATE INDEX patient_doctor ON patient(patient_id);
CREATE INDEX doctor_patient ON patient(doctor_id);
