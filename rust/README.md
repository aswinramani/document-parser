sample cda document
<name use="L">
    <prefix>Dr</prefix>
    <given>John</given>
    <given>Michael</given>   <!-- middle name — multiple given names possible -->
    <family>Doe</family>
    <suffix>Jr</suffix>
</name>

<addr use="H">
    <streetAddressLine>123 Main St</streetAddressLine>
    <city>Boston</city>
    <state>MA</state>
    <postalCode>02101</postalCode>
    <country>US</country>
</addr

<section>
  <!-- Section identification -->
  <templateId root="2.16.840.1.113883.10.20.22.2.5.1"/>
  <code code="11450-4" 
        displayName="Problem List" 
        codeSystem="2.16.840.1.113883.6.1" 
        codeSystemName="LOINC"/>
  <title>Problems</title>
  
  <entry typeCode="DRIV">
    <act classCode="ACT" moodCode="EVN">
      <templateId root="2.16.840.1.113883.10.20.22.4.3"/>
      
      <!-- Problem observation -->
      <entryRelationship typeCode="SUBJ">
        <observation classCode="OBS" moodCode="EVN">
          <templateId root="2.16.840.1.113883.10.20.22.4.4"/>
          
          <!-- Status -->
          <statusCode code="active"/>
          
          <!-- When problem started -->
          <effectiveTime>
            <low value="20180601"/>
            <high nullFlavor="UNK"/>
          </effectiveTime>
          
          <!-- The actual problem — SNOMED code -->
          <value xsi:type="CD"
                 code="44054006"
                 displayName="Diabetes mellitus type 2"
                 codeSystem="2.16.840.1.113883.6.96"
                 codeSystemName="SNOMED CT"/>
                 
          <!-- Severity -->
          <entryRelationship typeCode="SUBST">
            <observation classCode="OBS" moodCode="EVN">
              <value xsi:type="CD"
                     code="371924009"
                     displayName="Moderate to severe"
                     codeSystem="2.16.840.1.113883.6.96"/>
            </observation>
          </entryRelationship>
          
        </observation>
      </entryRelationship>
    </act>
  </entry>

  <!-- Missing data example — nullFlavor means unknown/missing -->
  <entry typeCode="DRIV">
    <act classCode="ACT" moodCode="EVN">
      <entryRelationship typeCode="SUBJ">
        <observation classCode="OBS" moodCode="EVN">
          <statusCode code="active"/>
          <effectiveTime>
            <low nullFlavor="UNK"/>  <!-- date unknown — real world messiness -->
          </effectiveTime>
          <value xsi:type="CD"
                 code="59621000"
                 displayName="Hypertension"
                 codeSystem="2.16.840.1.113883.6.96"
                 codeSystemName="SNOMED CT"/>
        </observation>
      </entryRelationship>
    </act>
  </entry>

</section>

