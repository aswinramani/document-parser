#!/usr/bin/env python3
"""
Generate sample C-CDA XML documents for parser testing and benchmarking.

Usage:
    python generate_samples.py --mode clear    # clears all docs
    python generate_samples.py --mode test     # ~10 varied documents for parser testing
    python generate_samples.py --mode bench    # 10,000 documents for benchmarking
    python generate_samples.py --mode bench --count 1000 --output path/to/dir
    python generate_samples.py --mode test --encrypt      # encrypt documents before writing
    python generate_samples.py --mode decrypt              # decrypt a directory of .xml.enc files

Encryption (--encrypt / --mode decrypt) requires the SAMPLE_DOC_ENCRYPTION_KEY
environment variable to hold a Fernet key. Generate one with:
    python -c "from cryptography.fernet import Fernet; print(Fernet.generate_key().decode())"
"""

import random
import uuid
import argparse
import os
from datetime import datetime, timedelta

from cryptography.fernet import Fernet
from dotenv import load_dotenv

# ---------------------------------------------------------------------------
# Data pools
# ---------------------------------------------------------------------------

GIVEN_NAMES = ["Henry", "Sarah", "John", "Mary", "James", "Patricia",
               "Robert", "Linda", "Michael", "Barbara", "William", "Susan"]

FAMILY_NAMES = ["Levin", "Smith", "Johnson", "Williams", "Brown",
                "Jones", "Garcia", "Miller", "Davis", "Wilson"]

GENDERS = [("M", "Male"), ("F", "Female")]

PROBLEM_CODES = [
    ("44054006",  "Diabetes mellitus type 2",    "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("59621000",  "Essential hypertension",       "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("195967001", "Asthma",                       "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("44054006",  "Chronic kidney disease",       "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("73211009",  "Diabetes mellitus",            "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("363746003", "Acute pharyngitis",            "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("22298006",  "Myocardial infarction",        "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("40275004",  "Contact dermatitis",           "2.16.840.1.113883.6.96", "SNOMED CT"),
]

ALLERGY_CODES = [
    ("91936005", "Allergy to penicillin",  "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("293586001","Allergy to aspirin",     "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("372687004","Allergy to amoxicillin", "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("418689008","Allergy to grass pollen","2.16.840.1.113883.6.96", "SNOMED CT"),
]

MEDICATION_CODES = [
    ("66493003",  "Theophylline",               "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("10312003",  "Prednisone preparation",      "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("91143003",  "Albuterol",                   "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("376209006", "Hydrochlorothiazide 25mg",    "2.16.840.1.113883.6.96", "SNOMED CT"),
    ("331646005", "Hydrocortisone cream",        "2.16.840.1.113883.6.96", "SNOMED CT"),
]

STATUS_CODES = ["active", "completed", "suspended"]

LOINC_DOCUMENT_CODES = [
    ("34133-9",  "Summarization of episode note"),
    ("11488-4",  "Consultation note"),
    ("18842-5",  "Discharge summary"),
    ("11506-3",  "Progress note"),
]

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def rand_uid():
    return str(uuid.uuid4())

def rand_root():
    return f"2.16.840.1.113883.19.{random.randint(1, 9999)}"

def rand_date(start_year=1940, end_year=1990):
    start = datetime(start_year, 1, 1)
    end = datetime(end_year, 12, 31)
    delta = end - start
    return (start + timedelta(days=random.randint(0, delta.days))).strftime("%Y%m%d")

def rand_recent_date():
    return rand_date(2010, 2024)

def rand_problem():
    return random.choice(PROBLEM_CODES)

def rand_allergy():
    return random.choice(ALLERGY_CODES)

def rand_medication():
    return random.choice(MEDICATION_CODES)

def rand_patient():
    given = random.choice(GIVEN_NAMES)
    family = random.choice(FAMILY_NAMES)
    gender_code, gender_display = random.choice(GENDERS)
    dob = rand_date(1930, 2000)
    patient_id = str(random.randint(10000, 99999))
    return given, family, gender_code, dob, patient_id

def rand_doc_code():
    return random.choice(LOINC_DOCUMENT_CODES)

# ---------------------------------------------------------------------------
# XML section builders
# ---------------------------------------------------------------------------

def problems_section(status="active"):
    code, display, code_system, code_system_name = rand_problem()
    onset = rand_recent_date()
    status_code = "active" if status == "active" else "completed"
    concern_status = "active" if status == "active" else "completed"

    return f"""
        <component>
            <section>
                <templateId root="2.16.840.1.113883.10.20.22.2.5.1"/>
                <templateId root="2.16.840.1.113883.10.20.22.2.5.1" extension="2015-08-01"/>
                <code code="11450-4" codeSystem="2.16.840.1.113883.6.1" codeSystemName="LOINC" displayName="Problem List"/>
                <title>Problem List</title>
                <text>Problem: {display} — Status: {status}</text>
                <entry>
                    <act classCode="ACT" moodCode="EVN">
                        <templateId root="2.16.840.1.113883.10.20.22.4.3"/>
                        <templateId root="2.16.840.1.113883.10.20.22.4.3" extension="2015-08-01"/>
                        <id root="{rand_uid()}"/>
                        <code code="CONC" codeSystem="2.16.840.1.113883.5.6"/>
                        <statusCode code="{concern_status}"/>
                        <effectiveTime>
                            <low value="{onset}"/>
                        </effectiveTime>
                        <entryRelationship typeCode="SUBJ">
                            <observation classCode="OBS" moodCode="EVN">
                                <templateId root="2.16.840.1.113883.10.20.22.4.4"/>
                                <templateId root="2.16.840.1.113883.10.20.22.4.4" extension="2015-08-01"/>
                                <id root="{rand_uid()}"/>
                                <code code="55607006" displayName="Problem"
                                    codeSystem="2.16.840.1.113883.6.96" codeSystemName="SNOMED CT"/>
                                <statusCode code="{status_code}"/>
                                <effectiveTime>
                                    <low value="{onset}"/>
                                </effectiveTime>
                                <value xsi:type="CD" code="{code}"
                                    codeSystem="{code_system}"
                                    codeSystemName="{code_system_name}"
                                    displayName="{display}"/>
                            </observation>
                        </entryRelationship>
                    </act>
                </entry>
            </section>
        </component>"""


def medications_section():
    code, display, code_system, code_system_name = rand_medication()
    dose = random.choice([10, 20, 25, 50, 100, 200, 500])
    unit = random.choice(["mg", "ml"])
    period = random.choice([6, 8, 12, 24])

    return f"""
        <component>
            <section>
                <templateId root="2.16.840.1.113883.10.20.22.2.1.1"/>
                <code code="10160-0" codeSystem="2.16.840.1.113883.6.1" codeSystemName="LOINC" displayName="Medications"/>
                <title>Medications</title>
                <text>{display} {dose}{unit} every {period}h</text>
                <entry>
                    <substanceAdministration classCode="SBADM" moodCode="EVN">
                        <templateId root="2.16.840.1.113883.10.20.22.4.16"/>
                        <id root="{rand_uid()}"/>
                        <statusCode code="active"/>
                        <effectiveTime xsi:type="PIVL_TS" institutionSpecified="true">
                            <period value="{period}" unit="h"/>
                        </effectiveTime>
                        <doseQuantity value="{dose}" unit="{unit}"/>
                        <consumable>
                            <manufacturedProduct classCode="MANU">
                                <templateId root="2.16.840.1.113883.10.20.22.4.23"/>
                                <manufacturedMaterial>
                                    <code code="{code}"
                                        codeSystem="{code_system}"
                                        codeSystemName="{code_system_name}"
                                        displayName="{display}"/>
                                </manufacturedMaterial>
                            </manufacturedProduct>
                        </consumable>
                    </substanceAdministration>
                </entry>
            </section>
        </component>"""


def allergies_section():
    code, display, code_system, code_system_name = rand_allergy()

    return f"""
        <component>
            <section>
                <templateId root="2.16.840.1.113883.10.20.22.2.6.1"/>
                <code code="48765-2" codeSystem="2.16.840.1.113883.6.1" codeSystemName="LOINC" displayName="Allergies"/>
                <title>Allergies and Adverse Reactions</title>
                <text>{display}</text>
                <entry>
                    <act classCode="ACT" moodCode="EVN">
                        <templateId root="2.16.840.1.113883.10.20.22.4.30"/>
                        <id root="{rand_uid()}"/>
                        <code code="CONC" codeSystem="2.16.840.1.113883.5.6"/>
                        <statusCode code="active"/>
                        <effectiveTime>
                            <low value="{rand_recent_date()}"/>
                        </effectiveTime>
                        <entryRelationship typeCode="SUBJ">
                            <observation classCode="OBS" moodCode="EVN">
                                <templateId root="2.16.840.1.113883.10.20.22.4.7"/>
                                <id root="{rand_uid()}"/>
                                <code code="ASSERTION" codeSystem="2.16.840.1.113883.5.4"/>
                                <statusCode code="completed"/>
                                <value xsi:type="CD" code="{code}"
                                    codeSystem="{code_system}"
                                    codeSystemName="{code_system_name}"
                                    displayName="{display}"/>
                            </observation>
                        </entryRelationship>
                    </act>
                </entry>
            </section>
        </component>"""


def vitals_section():
    height = round(random.uniform(1.50, 1.95), 2)
    weight = round(random.uniform(50.0, 120.0), 1)
    systolic = random.randint(100, 160)
    diastolic = random.randint(60, 100)
    obs_time = rand_recent_date() + str(random.randint(10, 16)) + "00"

    return f"""
        <component>
            <section>
                <templateId root="2.16.840.1.113883.10.20.22.2.4.1"/>
                <code code="8716-3" codeSystem="2.16.840.1.113883.6.1" codeSystemName="LOINC" displayName="Vital Signs"/>
                <title>Vital Signs</title>
                <text>Height: {height}m Weight: {weight}kg BP: {systolic}/{diastolic}</text>
                <entry>
                    <observation classCode="OBS" moodCode="EVN">
                        <templateId root="2.16.840.1.113883.10.20.22.4.27"/>
                        <id root="{rand_uid()}"/>
                        <code code="50373000" codeSystem="2.16.840.1.113883.6.96" codeSystemName="SNOMED CT" displayName="Body height measure"/>
                        <statusCode code="completed"/>
                        <effectiveTime value="{obs_time}"/>
                        <value xsi:type="PQ" value="{height}" unit="m"/>
                    </observation>
                </entry>
                <entry>
                    <observation classCode="OBS" moodCode="EVN">
                        <templateId root="2.16.840.1.113883.10.20.22.4.27"/>
                        <id root="{rand_uid()}"/>
                        <code code="363808001" codeSystem="2.16.840.1.113883.6.96" codeSystemName="SNOMED CT" displayName="Body weight measure"/>
                        <statusCode code="completed"/>
                        <effectiveTime value="{obs_time}"/>
                        <value xsi:type="PQ" value="{weight}" unit="kg"/>
                    </observation>
                </entry>
                <entry>
                    <observation classCode="OBS" moodCode="EVN">
                        <templateId root="2.16.840.1.113883.10.20.22.4.27"/>
                        <id root="{rand_uid()}"/>
                        <code code="251076008" codeSystem="2.16.840.1.113883.6.96" codeSystemName="SNOMED CT" displayName="Cuff blood pressure"/>
                        <statusCode code="completed"/>
                        <effectiveTime value="{obs_time}"/>
                        <entryRelationship typeCode="COMP">
                            <observation classCode="OBS" moodCode="EVN">
                                <code code="271649006" codeSystem="2.16.840.1.113883.6.96" codeSystemName="SNOMED CT" displayName="Systolic BP"/>
                                <statusCode code="completed"/>
                                <value xsi:type="PQ" value="{systolic}" unit="mm[Hg]"/>
                            </observation>
                        </entryRelationship>
                        <entryRelationship typeCode="COMP">
                            <observation classCode="OBS" moodCode="EVN">
                                <code code="271650006" codeSystem="2.16.840.1.113883.6.96" codeSystemName="SNOMED CT" displayName="Diastolic BP"/>
                                <statusCode code="completed"/>
                                <value xsi:type="PQ" value="{diastolic}" unit="mm[Hg]"/>
                            </observation>
                        </entryRelationship>
                    </observation>
                </entry>
            </section>
        </component>"""


# ---------------------------------------------------------------------------
# Full document builder
# ---------------------------------------------------------------------------

def generate_document(sections=None):
    given, family, gender_code, dob, patient_id = rand_patient()
    doc_code, doc_display = rand_doc_code()
    doc_date = rand_recent_date()
    doc_id = rand_uid()

    if sections is None:
        sections = [problems_section()]

    body = "\n".join(sections)

    return f"""<?xml version="1.0" encoding="UTF-8"?>
<ClinicalDocument xmlns="urn:hl7-org:v3"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:voc="urn:hl7-org:v3/voc">
    <typeId root="2.16.840.1.113883.1.3" extension="POCD_HD000040"/>
    <templateId root="2.16.840.1.113883.10.20.22.1.1"/>
    <id root="{doc_id}"/>
    <code code="{doc_code}" codeSystem="2.16.840.1.113883.6.1"
        codeSystemName="LOINC" displayName="{doc_display}"/>
    <title>{doc_display}</title>
    <effectiveTime value="{doc_date}"/>
    <confidentialityCode code="N" codeSystem="2.16.840.1.113883.5.25"/>
    <languageCode code="en-US"/>
    <recordTarget>
        <patientRole>
            <id extension="{patient_id}" root="2.16.840.1.113883.19.5"/>
            <patient>
                <name>
                    <given>{given}</given>
                    <family>{family}</family>
                </name>
                <administrativeGenderCode code="{gender_code}" codeSystem="2.16.840.1.113883.5.1"/>
                <birthTime value="{dob}"/>
            </patient>
        </patientRole>
    </recordTarget>
    <component>
        <structuredBody>
            {body}
        </structuredBody>
    </component>
</ClinicalDocument>"""


# ---------------------------------------------------------------------------
# Encryption
# ---------------------------------------------------------------------------

def get_fernet():
    key = os.environ.get("SAMPLE_DOC_ENCRYPTION_KEY")
    if not key:
        raise SystemExit(
            "SAMPLE_DOC_ENCRYPTION_KEY is not set. Generate one with:\n"
            '  python -c "from cryptography.fernet import Fernet; print(Fernet.generate_key().decode())"'
        )
    return Fernet(key.encode())


def write_document(path, content, fernet):
    if fernet:
        path += ".enc"
        data = fernet.encrypt(content.encode("utf-8"))
    else:
        data = content.encode("utf-8")
    with open(path, "wb") as f:
        f.write(data)
    return path


# ---------------------------------------------------------------------------
# Modes
# ---------------------------------------------------------------------------

def generate_test_documents(output_dir, fernet=None):
    docs = [
        ("active_problem.xml",   generate_document([problems_section("active")])),
        ("resolved_problem.xml", generate_document([problems_section("resolved")])),
        ("medications.xml",      generate_document([medications_section()])),
        ("allergies.xml",        generate_document([allergies_section()])),
        ("vitals.xml",           generate_document([vitals_section()])),
        ("multi_section.xml",    generate_document([
            problems_section("active"),
            medications_section(),
            allergies_section(),
            vitals_section(),
        ])),
        ("empty_sections.xml",   generate_document([])),
        ("two_problems.xml",     generate_document([
            problems_section("active"),
            problems_section("resolved"),
        ])),
    ]

    os.makedirs(output_dir, exist_ok=True)
    for filename, content in docs:
        path = write_document(os.path.join(output_dir, filename), content, fernet)
        print(f"  wrote {path}")

    print(f"\nGenerated {len(docs)} test documents in {output_dir}")


def generate_bench_documents(output_dir, count, fernet=None):
    os.makedirs(output_dir, exist_ok=True)
    section_builders = [
        lambda: problems_section("active"),
        lambda: problems_section("resolved"),
        lambda: medications_section(),
        lambda: allergies_section(),
        lambda: vitals_section(),
    ]

    for i in range(count):
        num_sections = random.randint(1, 3)
        sections = [random.choice(section_builders)() for _ in range(num_sections)]
        content = generate_document(sections)
        write_document(os.path.join(output_dir, f"doc_{i:06d}.xml"), content, fernet)

        if (i + 1) % 1000 == 0:
            print(f"  {i + 1}/{count} documents written...")

    print(f"\nGenerated {count} bench documents in {output_dir}")


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def clear_directory(output_dir):
    if not os.path.exists(output_dir):
        print(f"Nothing to clear — {output_dir} does not exist")
        return
    files = [f for f in os.listdir(output_dir) if f.endswith(".xml") or f.endswith(".xml.enc")]
    for f in files:
        os.remove(os.path.join(output_dir, f))
    print(f"Cleared {len(files)} files from {output_dir}")


def decrypt_documents(output_dir, fernet):
    if not os.path.exists(output_dir):
        print(f"Nothing to decrypt — {output_dir} does not exist")
        return
    files = [f for f in os.listdir(output_dir) if f.endswith(".xml.enc")]
    for f in files:
        enc_path = os.path.join(output_dir, f)
        with open(enc_path, "rb") as fh:
            plaintext = fernet.decrypt(fh.read())
        out_path = enc_path[: -len(".enc")]
        with open(out_path, "wb") as fh:
            fh.write(plaintext)
        print(f"  decrypted {out_path}")

    print(f"\nDecrypted {len(files)} documents in {output_dir}")


def main():
    load_dotenv()

    parser = argparse.ArgumentParser(description="Generate sample C-CDA XML documents")
    parser.add_argument("--mode", choices=["test", "bench", "clear", "decrypt"], default="test",
                        help="test: varied documents | bench: bulk for benchmarking | "
                             "clear: delete generated files | decrypt: decrypt .xml.enc files in place")
    parser.add_argument("--count", type=int, default=10000,
                        help="number of documents for bench mode (default: 10000)")
    parser.add_argument("--output", type=str, default=None,
                        help="output directory (default: sample-documents/generated or benchmarks/samples)")
    parser.add_argument("--encrypt", action="store_true",
                        help="encrypt documents in memory before writing (requires SAMPLE_DOC_ENCRYPTION_KEY)")
    args = parser.parse_args()

    if args.output:
        output_dir = args.output
    elif args.mode in ("test", "clear", "decrypt"):
        output_dir = os.path.join(os.path.dirname(__file__), "..", "sample-documents", "generated")
    else:
        output_dir = os.path.join(os.path.dirname(__file__), "..", "benchmarks", "samples")

    print(f"Mode: {args.mode}")
    print(f"Output: {output_dir}")

    fernet = get_fernet() if (args.encrypt or args.mode == "decrypt") else None

    if args.mode == "test":
        generate_test_documents(output_dir, fernet)
    elif args.mode == "bench":
        generate_bench_documents(output_dir, args.count, fernet)
    elif args.mode == "decrypt":
        decrypt_documents(output_dir, fernet)
    else:
        clear_directory(output_dir)


if __name__ == "__main__":
    main()
