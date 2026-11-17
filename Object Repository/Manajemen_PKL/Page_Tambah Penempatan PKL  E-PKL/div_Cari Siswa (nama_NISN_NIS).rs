<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Cari Siswa (nama_NISN_NIS)</name>
   <tag></tag>
   <elementGuidId>c6d5fb31-d72b-4377-856b-afef8cb46b14</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>.card-body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' card-body ')]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#content div >> internal:has-text=&quot;Cari Siswa (nama/NISN/NIS) Cari Pilih Siswa aktif Developer E-PKL Siswa · 999999&quot;i >> nth=1</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>26b15edf-f0b4-482a-a962-5a456e545fc6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>card-body</value>
      <webElementGuid>95d242f1-ac36-4131-9c1e-cb54a01fb2b9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
            
            
            
    
        Cari Siswa (nama/NISN/NIS)
        
            
            Cari
        
        
            Pilih Siswa aktif
                                                        
                    Developer E-PKL Siswa · 9999999999                
                                            
                    niha natus · 0000000012                
                                            
                    siswa testing baru · 1414090988                
                                            
                    Siswa Uji EPKL Registrasi · 8888888801                
                                            
                    siswa uji testing · 5432112345                
                                            
                    siswi testing baru · 99996666                
                                            
                    siswi uji · 00111001                
                    
        
            Sebelumnya
            1 / 1
            Berikutnya
            Pilihan menggunakan ID profil Siswa.
        
    
    
        Periode PKL
        
            Pilih periode persiapan/berjalan
                            
                    Periode Aktif · persiapan                
                    
    

    Instansi 
    
        Pilih Instansi aktif
                    
                PSDKU Nganjuk            
                    
                pt bajang12            
            
    Pilih Instansi dan periode untuk melihat kuota.


    Cari Guru (nama/NIP)
    
        
        Cari
    
    
        Pilih Guru aktif
                                        
                guruepkl · developer.guru            
            
    
        Sebelumnya
        1 / 1
        Berikutnya
        Pilihan menggunakan ID profil Guru.
    


    PIC Instansi (opsional)
    
        Tanpa PIC
            


    
        Tanggal mulai
        
    
    
        Tanggal selesai
        
    


document.addEventListener('DOMContentLoaded', function () {
    const form = document.getElementById('placement-form');
    const errorBox = document.getElementById('placementReferenceError');
    const studentSelect = document.getElementById('student-select');
    const teacherSelect = document.getElementById('teacher-select');
    const periodSelect = document.getElementById('period-select');
    const instanceSelect = document.getElementById('instance-select');
    const picSelect = document.getElementById('pic-select');
    let referenceData = {&quot;siswa&quot;:[{&quot;id&quot;:1,&quot;profile_id&quot;:1,&quot;nama_lengkap&quot;:&quot;Developer E-PKL Siswa&quot;,&quot;nisn&quot;:&quot;9999999999&quot;,&quot;nis&quot;:&quot;DEV-SISWA-001&quot;,&quot;username&quot;:&quot;developer.siswa&quot;},{&quot;id&quot;:8,&quot;profile_id&quot;:8,&quot;nama_lengkap&quot;:&quot;niha natus&quot;,&quot;nisn&quot;:&quot;0000000012&quot;,&quot;nis&quot;:&quot;00430506&quot;,&quot;username&quot;:&quot;niha&quot;},{&quot;id&quot;:4,&quot;profile_id&quot;:4,&quot;nama_lengkap&quot;:&quot;siswa testing baru&quot;,&quot;nisn&quot;:&quot;1414090988&quot;,&quot;nis&quot;:&quot;0776543&quot;,&quot;username&quot;:&quot;siswa.baru&quot;},{&quot;id&quot;:2,&quot;profile_id&quot;:2,&quot;nama_lengkap&quot;:&quot;Siswa Uji EPKL Registrasi&quot;,&quot;nisn&quot;:&quot;8888888801&quot;,&quot;nis&quot;:null,&quot;username&quot;:&quot;8888888801&quot;},{&quot;id&quot;:6,&quot;profile_id&quot;:6,&quot;nama_lengkap&quot;:&quot;siswa uji testing&quot;,&quot;nisn&quot;:&quot;5432112345&quot;,&quot;nis&quot;:&quot;00976711&quot;,&quot;username&quot;:&quot;siswauji&quot;},{&quot;id&quot;:5,&quot;profile_id&quot;:5,&quot;nama_lengkap&quot;:&quot;siswi testing baru&quot;,&quot;nisn&quot;:&quot;99996666&quot;,&quot;nis&quot;:&quot;0099876&quot;,&quot;username&quot;:&quot;siswi.baru&quot;},{&quot;id&quot;:7,&quot;profile_id&quot;:7,&quot;nama_lengkap&quot;:&quot;siswi uji&quot;,&quot;nisn&quot;:&quot;00111001&quot;,&quot;nis&quot;:&quot;12340987&quot;,&quot;username&quot;:&quot;siswi uji&quot;}],&quot;guru&quot;:[{&quot;id&quot;:1,&quot;profile_id&quot;:1,&quot;nama_lengkap&quot;:&quot;guruepkl&quot;,&quot;nip&quot;:&quot;developer.guru&quot;,&quot;username&quot;:&quot;developer.guru&quot;}],&quot;selected&quot;:{&quot;siswa&quot;:null,&quot;guru&quot;:null},&quot;pagination&quot;:{&quot;per_page&quot;:20,&quot;siswa&quot;:{&quot;current_page&quot;:1,&quot;per_page&quot;:20,&quot;total&quot;:7,&quot;last_page&quot;:1},&quot;guru&quot;:{&quot;current_page&quot;:1,&quot;per_page&quot;:20,&quot;total&quot;:1,&quot;last_page&quot;:1}},&quot;periode&quot;:[{&quot;id&quot;:1,&quot;nama&quot;:&quot;Periode Aktif&quot;,&quot;tanggal_mulai&quot;:&quot;2026-11-16&quot;,&quot;tanggal_selesai&quot;:&quot;2027-02-16&quot;,&quot;status&quot;:&quot;persiapan&quot;}],&quot;instansi&quot;:[{&quot;id&quot;:3,&quot;nama&quot;:&quot;PSDKU Nganjuk&quot;},{&quot;id&quot;:2,&quot;nama&quot;:&quot;pt bajang12&quot;}],&quot;pic_instansi&quot;:[],&quot;kuota&quot;:null};
    let referencesReady = true;
    let referenceRequest = 0;

    function updatePageLabels() {
        ['siswa', 'guru'].forEach(function (key) {
            const data = referenceData.pagination?.[key];
            const label = document.getElementById(key === 'siswa' ? 'student-page' : 'teacher-page');
            const previous = document.getElementById(key === 'siswa' ? 'student-prev' : 'teacher-prev');
            const next = document.getElementById(key === 'siswa' ? 'student-next' : 'teacher-next');
            if (!label || !data) return;
            label.textContent = data.current_page + ' / ' + Math.max(data.last_page, 1);
            previous.disabled = data.current_page &lt;= 1;
            next.disabled = data.current_page >= data.last_page;
        });
    }

    function setOptions(select, rows, selected, label, preserve) {
        if (!select) return;
        const oldValue = preserve ? select.value : '';
        select.replaceChildren(new Option(label, ''));
        const visible = Array.isArray(rows) ? rows.slice() : [];
        if (selected &amp;&amp; !visible.some((row) => String(row.id) === String(selected.id))) {
            visible.unshift(selected);
        }
        visible.forEach(function (row) {
            const text = select.id === 'student-select'
                ? (row.nama_lengkap || '-') + ' · ' + (row.nisn || '-')
                : (row.nama_lengkap || '-') + ' · ' + (row.nip || '-');
            select.add(new Option(text, row.id));
        });
        if (oldValue &amp;&amp; Array.from(select.options).some((option) => option.value === oldValue)) {
            select.value = oldValue;
        } else if (selected &amp;&amp; String(selected.id) === oldValue) {
            select.value = oldValue;
        }
    }

    function updateQuota() {
        const target = document.getElementById('placement-quota');
        const quota = referenceData.kuota;
        if (!target) return;
        if (!quota) {
            target.textContent = 'Pilih Instansi dan periode untuk melihat kuota.';
        } else if (quota.tanpa_batas) {
            target.textContent = 'Kuota tidak dibatasi; terpakai ' + quota.terpakai + '.';
        } else {
            target.textContent = 'Kuota ' + quota.kuota_siswa + '; terpakai ' + quota.terpakai + '; sisa ' + quota.sisa + '.';
        }
    }

    function updateDateRange() {
        const start = document.getElementById('placement-start');
        const end = document.getElementById('placement-end');
        let periodStart = &quot;&quot;;
        let periodEnd = &quot;&quot;;
        if (periodSelect) {
            const option = periodSelect.options[periodSelect.selectedIndex];
            periodStart = option?.dataset.start || '';
            periodEnd = option?.dataset.end || '';
        }
        start.min = periodStart;
        start.max = periodEnd;
        end.min = start.value || periodStart;
        end.max = periodEnd;
    }

    async function loadReferences(change) {
        const requestId = ++referenceRequest;
        referencesReady = false;
        errorBox.classList.add('d-none');
        const currentStudent = studentSelect?.value || '';
        const currentTeacher = teacherSelect?.value || '';
        const params = new URLSearchParams({
            page: 'referensi-penempatan-pkl',
            per_page: '20',
            siswa_search: document.getElementById('student-search')?.value || '',
            guru_search: document.getElementById('teacher-search')?.value || '',
            siswa_page: String(change.siswaPage ?? referenceData.pagination?.siswa?.current_page ?? 1),
            guru_page: String(change.guruPage ?? referenceData.pagination?.guru?.current_page ?? 1),
            instansi_id: instanceSelect?.value || '',
            periode_id: periodSelect?.value || '',
            siswa_id: currentStudent,
            guru_id: currentTeacher
        });
        try {
            const response = await fetch('?' + params.toString(), { headers: { Accept: 'application/json' } });
            const body = await response.json();
            if (requestId !== referenceRequest) return;
            if (!response.ok || body.success !== true || !body.data) {
                throw new Error(body.message || 'Referensi tidak dapat dimuat.');
            }
            referenceData = body.data;
            setOptions(studentSelect, referenceData.siswa, referenceData.selected?.siswa, 'Pilih Siswa aktif', true);
            setOptions(teacherSelect, referenceData.guru, referenceData.selected?.guru, 'Pilih Guru aktif', true);
            if (picSelect) {
                const oldPic = picSelect.value;
                picSelect.replaceChildren(new Option('Tanpa PIC', ''));
                (referenceData.pic_instansi || []).forEach((pic) => {
                    picSelect.add(new Option(pic.nama + (pic.jabatan ? ' · ' + pic.jabatan : ''), pic.id));
                });
                if (Array.from(picSelect.options).some((option) => option.value === oldPic)) picSelect.value = oldPic;
            }
            updatePageLabels();
            updateQuota();
            referencesReady = true;
        } catch (error) {
            if (requestId !== referenceRequest) return;
            errorBox.textContent = error.message || 'Referensi tidak dapat dimuat.';
            errorBox.classList.remove('d-none');
        }
    }

    document.getElementById('student-search-button')?.addEventListener('click', () => loadReferences({ siswaPage: 1 }));
    document.getElementById('teacher-search-button')?.addEventListener('click', () => loadReferences({ guruPage: 1 }));
    document.getElementById('student-search')?.addEventListener('keydown', (event) => {
        if (event.key === 'Enter') { event.preventDefault(); loadReferences({ siswaPage: 1 }); }
    });
    document.getElementById('teacher-search')?.addEventListener('keydown', (event) => {
        if (event.key === 'Enter') { event.preventDefault(); loadReferences({ guruPage: 1 }); }
    });
    document.getElementById('student-prev')?.addEventListener('click', () => loadReferences({ siswaPage: Math.max(1, (referenceData.pagination?.siswa?.current_page || 1) - 1) }));
    document.getElementById('student-next')?.addEventListener('click', () => loadReferences({ siswaPage: (referenceData.pagination?.siswa?.current_page || 1) + 1 }));
    document.getElementById('teacher-prev')?.addEventListener('click', () => loadReferences({ guruPage: Math.max(1, (referenceData.pagination?.guru?.current_page || 1) - 1) }));
    document.getElementById('teacher-next')?.addEventListener('click', () => loadReferences({ guruPage: (referenceData.pagination?.guru?.current_page || 1) + 1 }));
    instanceSelect?.addEventListener('change', () => loadReferences({}));
    periodSelect?.addEventListener('change', () => { updateDateRange(); loadReferences({}); });
    document.getElementById('placement-start')?.addEventListener('change', updateDateRange);
    updatePageLabels();
    updateQuota();
    updateDateRange();
    form?.addEventListener('submit', function (event) {
        if (!referencesReady) {
            event.preventDefault();
            errorBox.textContent = 'Form tidak dapat dikirim sebelum data referensi berhasil dimuat.';
            errorBox.classList.remove('d-none');
        }
    });
});

            
                Batal
                
                    Simpan Penempatan
                
            
        
    </value>
      <webElementGuid>5d948b35-d770-4ee0-b511-1f6bbf98ad6b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-95cb1e8b826bbd5ba938d5f1dcf64081</value>
      <webElementGuid>01f1fbf9-4005-4d5b-abfc-55992c3a83d4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' card-body ')]</value>
      <webElementGuid>481600ab-163d-48fb-ba6d-f7cc9ddd0374</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//*[@class and contains(concat(' ', normalize-space(@class), ' '), ' card-body ')]</value>
      <webElementGuid>5b383ba5-6e0a-4a15-85d7-e605ec0eda87</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
        
            
            
            
    
        Cari Siswa (nama/NISN/NIS)
        
            
            Cari
        
        
            Pilih Siswa aktif
                                                        
                    Developer E-PKL Siswa · 9999999999                
                                            
                    niha natus · 0000000012                
                                            
                    siswa testing baru · 1414090988                
                                            
                    Siswa Uji EPKL Registrasi · 8888888801                
                                            
                    siswa uji testing · 5432112345                
                                            
                    siswi testing baru · 99996666                
                                            
                    siswi uji · 00111001                
                    
        
            Sebelumnya
            1 / 1
            Berikutnya
            Pilihan menggunakan ID profil Siswa.
        
    
    
        Periode PKL
        
            Pilih periode persiapan/berjalan
                            
                    Periode Aktif · persiapan                
                    
    

    Instansi 
    
        Pilih Instansi aktif
                    
                PSDKU Nganjuk            
                    
                pt bajang12            
            
    Pilih Instansi dan periode untuk melihat kuota.


    Cari Guru (nama/NIP)
    
        
        Cari
    
    
        Pilih Guru aktif
                                        
                guruepkl · developer.guru            
            
    
        Sebelumnya
        1 / 1
        Berikutnya
        Pilihan menggunakan ID profil Guru.
    


    PIC Instansi (opsional)
    
        Tanpa PIC
            


    
        Tanggal mulai
        
    
    
        Tanggal selesai
        
    


document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, function () {
    const form = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-form&quot; , &quot;'&quot; , &quot;);
    const errorBox = document.getElementById(&quot; , &quot;'&quot; , &quot;placementReferenceError&quot; , &quot;'&quot; , &quot;);
    const studentSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;student-select&quot; , &quot;'&quot; , &quot;);
    const teacherSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-select&quot; , &quot;'&quot; , &quot;);
    const periodSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;period-select&quot; , &quot;'&quot; , &quot;);
    const instanceSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;instance-select&quot; , &quot;'&quot; , &quot;);
    const picSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;pic-select&quot; , &quot;'&quot; , &quot;);
    let referenceData = {&quot;siswa&quot;:[{&quot;id&quot;:1,&quot;profile_id&quot;:1,&quot;nama_lengkap&quot;:&quot;Developer E-PKL Siswa&quot;,&quot;nisn&quot;:&quot;9999999999&quot;,&quot;nis&quot;:&quot;DEV-SISWA-001&quot;,&quot;username&quot;:&quot;developer.siswa&quot;},{&quot;id&quot;:8,&quot;profile_id&quot;:8,&quot;nama_lengkap&quot;:&quot;niha natus&quot;,&quot;nisn&quot;:&quot;0000000012&quot;,&quot;nis&quot;:&quot;00430506&quot;,&quot;username&quot;:&quot;niha&quot;},{&quot;id&quot;:4,&quot;profile_id&quot;:4,&quot;nama_lengkap&quot;:&quot;siswa testing baru&quot;,&quot;nisn&quot;:&quot;1414090988&quot;,&quot;nis&quot;:&quot;0776543&quot;,&quot;username&quot;:&quot;siswa.baru&quot;},{&quot;id&quot;:2,&quot;profile_id&quot;:2,&quot;nama_lengkap&quot;:&quot;Siswa Uji EPKL Registrasi&quot;,&quot;nisn&quot;:&quot;8888888801&quot;,&quot;nis&quot;:null,&quot;username&quot;:&quot;8888888801&quot;},{&quot;id&quot;:6,&quot;profile_id&quot;:6,&quot;nama_lengkap&quot;:&quot;siswa uji testing&quot;,&quot;nisn&quot;:&quot;5432112345&quot;,&quot;nis&quot;:&quot;00976711&quot;,&quot;username&quot;:&quot;siswauji&quot;},{&quot;id&quot;:5,&quot;profile_id&quot;:5,&quot;nama_lengkap&quot;:&quot;siswi testing baru&quot;,&quot;nisn&quot;:&quot;99996666&quot;,&quot;nis&quot;:&quot;0099876&quot;,&quot;username&quot;:&quot;siswi.baru&quot;},{&quot;id&quot;:7,&quot;profile_id&quot;:7,&quot;nama_lengkap&quot;:&quot;siswi uji&quot;,&quot;nisn&quot;:&quot;00111001&quot;,&quot;nis&quot;:&quot;12340987&quot;,&quot;username&quot;:&quot;siswi uji&quot;}],&quot;guru&quot;:[{&quot;id&quot;:1,&quot;profile_id&quot;:1,&quot;nama_lengkap&quot;:&quot;guruepkl&quot;,&quot;nip&quot;:&quot;developer.guru&quot;,&quot;username&quot;:&quot;developer.guru&quot;}],&quot;selected&quot;:{&quot;siswa&quot;:null,&quot;guru&quot;:null},&quot;pagination&quot;:{&quot;per_page&quot;:20,&quot;siswa&quot;:{&quot;current_page&quot;:1,&quot;per_page&quot;:20,&quot;total&quot;:7,&quot;last_page&quot;:1},&quot;guru&quot;:{&quot;current_page&quot;:1,&quot;per_page&quot;:20,&quot;total&quot;:1,&quot;last_page&quot;:1}},&quot;periode&quot;:[{&quot;id&quot;:1,&quot;nama&quot;:&quot;Periode Aktif&quot;,&quot;tanggal_mulai&quot;:&quot;2026-11-16&quot;,&quot;tanggal_selesai&quot;:&quot;2027-02-16&quot;,&quot;status&quot;:&quot;persiapan&quot;}],&quot;instansi&quot;:[{&quot;id&quot;:3,&quot;nama&quot;:&quot;PSDKU Nganjuk&quot;},{&quot;id&quot;:2,&quot;nama&quot;:&quot;pt bajang12&quot;}],&quot;pic_instansi&quot;:[],&quot;kuota&quot;:null};
    let referencesReady = true;
    let referenceRequest = 0;

    function updatePageLabels() {
        [&quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;guru&quot; , &quot;'&quot; , &quot;].forEach(function (key) {
            const data = referenceData.pagination?.[key];
            const label = document.getElementById(key === &quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;student-page&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;teacher-page&quot; , &quot;'&quot; , &quot;);
            const previous = document.getElementById(key === &quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;student-prev&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;teacher-prev&quot; , &quot;'&quot; , &quot;);
            const next = document.getElementById(key === &quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;student-next&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;teacher-next&quot; , &quot;'&quot; , &quot;);
            if (!label || !data) return;
            label.textContent = data.current_page + &quot; , &quot;'&quot; , &quot; / &quot; , &quot;'&quot; , &quot; + Math.max(data.last_page, 1);
            previous.disabled = data.current_page &lt;= 1;
            next.disabled = data.current_page >= data.last_page;
        });
    }

    function setOptions(select, rows, selected, label, preserve) {
        if (!select) return;
        const oldValue = preserve ? select.value : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        select.replaceChildren(new Option(label, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));
        const visible = Array.isArray(rows) ? rows.slice() : [];
        if (selected &amp;&amp; !visible.some((row) => String(row.id) === String(selected.id))) {
            visible.unshift(selected);
        }
        visible.forEach(function (row) {
            const text = select.id === &quot; , &quot;'&quot; , &quot;student-select&quot; , &quot;'&quot; , &quot;
                ? (row.nama_lengkap || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; · &quot; , &quot;'&quot; , &quot; + (row.nisn || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)
                : (row.nama_lengkap || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; · &quot; , &quot;'&quot; , &quot; + (row.nip || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;);
            select.add(new Option(text, row.id));
        });
        if (oldValue &amp;&amp; Array.from(select.options).some((option) => option.value === oldValue)) {
            select.value = oldValue;
        } else if (selected &amp;&amp; String(selected.id) === oldValue) {
            select.value = oldValue;
        }
    }

    function updateQuota() {
        const target = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-quota&quot; , &quot;'&quot; , &quot;);
        const quota = referenceData.kuota;
        if (!target) return;
        if (!quota) {
            target.textContent = &quot; , &quot;'&quot; , &quot;Pilih Instansi dan periode untuk melihat kuota.&quot; , &quot;'&quot; , &quot;;
        } else if (quota.tanpa_batas) {
            target.textContent = &quot; , &quot;'&quot; , &quot;Kuota tidak dibatasi; terpakai &quot; , &quot;'&quot; , &quot; + quota.terpakai + &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;;
        } else {
            target.textContent = &quot; , &quot;'&quot; , &quot;Kuota &quot; , &quot;'&quot; , &quot; + quota.kuota_siswa + &quot; , &quot;'&quot; , &quot;; terpakai &quot; , &quot;'&quot; , &quot; + quota.terpakai + &quot; , &quot;'&quot; , &quot;; sisa &quot; , &quot;'&quot; , &quot; + quota.sisa + &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;;
        }
    }

    function updateDateRange() {
        const start = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-start&quot; , &quot;'&quot; , &quot;);
        const end = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-end&quot; , &quot;'&quot; , &quot;);
        let periodStart = &quot;&quot;;
        let periodEnd = &quot;&quot;;
        if (periodSelect) {
            const option = periodSelect.options[periodSelect.selectedIndex];
            periodStart = option?.dataset.start || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            periodEnd = option?.dataset.end || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        }
        start.min = periodStart;
        start.max = periodEnd;
        end.min = start.value || periodStart;
        end.max = periodEnd;
    }

    async function loadReferences(change) {
        const requestId = ++referenceRequest;
        referencesReady = false;
        errorBox.classList.add(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
        const currentStudent = studentSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const currentTeacher = teacherSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const params = new URLSearchParams({
            page: &quot; , &quot;'&quot; , &quot;referensi-penempatan-pkl&quot; , &quot;'&quot; , &quot;,
            per_page: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;,
            siswa_search: document.getElementById(&quot; , &quot;'&quot; , &quot;student-search&quot; , &quot;'&quot; , &quot;)?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            guru_search: document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-search&quot; , &quot;'&quot; , &quot;)?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            siswa_page: String(change.siswaPage ?? referenceData.pagination?.siswa?.current_page ?? 1),
            guru_page: String(change.guruPage ?? referenceData.pagination?.guru?.current_page ?? 1),
            instansi_id: instanceSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            periode_id: periodSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            siswa_id: currentStudent,
            guru_id: currentTeacher
        });
        try {
            const response = await fetch(&quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; + params.toString(), { headers: { Accept: &quot; , &quot;'&quot; , &quot;application/json&quot; , &quot;'&quot; , &quot; } });
            const body = await response.json();
            if (requestId !== referenceRequest) return;
            if (!response.ok || body.success !== true || !body.data) {
                throw new Error(body.message || &quot; , &quot;'&quot; , &quot;Referensi tidak dapat dimuat.&quot; , &quot;'&quot; , &quot;);
            }
            referenceData = body.data;
            setOptions(studentSelect, referenceData.siswa, referenceData.selected?.siswa, &quot; , &quot;'&quot; , &quot;Pilih Siswa aktif&quot; , &quot;'&quot; , &quot;, true);
            setOptions(teacherSelect, referenceData.guru, referenceData.selected?.guru, &quot; , &quot;'&quot; , &quot;Pilih Guru aktif&quot; , &quot;'&quot; , &quot;, true);
            if (picSelect) {
                const oldPic = picSelect.value;
                picSelect.replaceChildren(new Option(&quot; , &quot;'&quot; , &quot;Tanpa PIC&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));
                (referenceData.pic_instansi || []).forEach((pic) => {
                    picSelect.add(new Option(pic.nama + (pic.jabatan ? &quot; , &quot;'&quot; , &quot; · &quot; , &quot;'&quot; , &quot; + pic.jabatan : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;), pic.id));
                });
                if (Array.from(picSelect.options).some((option) => option.value === oldPic)) picSelect.value = oldPic;
            }
            updatePageLabels();
            updateQuota();
            referencesReady = true;
        } catch (error) {
            if (requestId !== referenceRequest) return;
            errorBox.textContent = error.message || &quot; , &quot;'&quot; , &quot;Referensi tidak dapat dimuat.&quot; , &quot;'&quot; , &quot;;
            errorBox.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
        }
    }

    document.getElementById(&quot; , &quot;'&quot; , &quot;student-search-button&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ siswaPage: 1 }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-search-button&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ guruPage: 1 }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;student-search&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;, (event) => {
        if (event.key === &quot; , &quot;'&quot; , &quot;Enter&quot; , &quot;'&quot; , &quot;) { event.preventDefault(); loadReferences({ siswaPage: 1 }); }
    });
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-search&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;, (event) => {
        if (event.key === &quot; , &quot;'&quot; , &quot;Enter&quot; , &quot;'&quot; , &quot;) { event.preventDefault(); loadReferences({ guruPage: 1 }); }
    });
    document.getElementById(&quot; , &quot;'&quot; , &quot;student-prev&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ siswaPage: Math.max(1, (referenceData.pagination?.siswa?.current_page || 1) - 1) }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;student-next&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ siswaPage: (referenceData.pagination?.siswa?.current_page || 1) + 1 }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-prev&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ guruPage: Math.max(1, (referenceData.pagination?.guru?.current_page || 1) - 1) }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-next&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ guruPage: (referenceData.pagination?.guru?.current_page || 1) + 1 }));
    instanceSelect?.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, () => loadReferences({}));
    periodSelect?.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, () => { updateDateRange(); loadReferences({}); });
    document.getElementById(&quot; , &quot;'&quot; , &quot;placement-start&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, updateDateRange);
    updatePageLabels();
    updateQuota();
    updateDateRange();
    form?.addEventListener(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function (event) {
        if (!referencesReady) {
            event.preventDefault();
            errorBox.textContent = &quot; , &quot;'&quot; , &quot;Form tidak dapat dikirim sebelum data referensi berhasil dimuat.&quot; , &quot;'&quot; , &quot;;
            errorBox.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
        }
    });
});

            
                Batal
                
                    Simpan Penempatan
                
            
        
    &quot;) or . = concat(&quot;
        
            
            
            
    
        Cari Siswa (nama/NISN/NIS)
        
            
            Cari
        
        
            Pilih Siswa aktif
                                                        
                    Developer E-PKL Siswa · 9999999999                
                                            
                    niha natus · 0000000012                
                                            
                    siswa testing baru · 1414090988                
                                            
                    Siswa Uji EPKL Registrasi · 8888888801                
                                            
                    siswa uji testing · 5432112345                
                                            
                    siswi testing baru · 99996666                
                                            
                    siswi uji · 00111001                
                    
        
            Sebelumnya
            1 / 1
            Berikutnya
            Pilihan menggunakan ID profil Siswa.
        
    
    
        Periode PKL
        
            Pilih periode persiapan/berjalan
                            
                    Periode Aktif · persiapan                
                    
    

    Instansi 
    
        Pilih Instansi aktif
                    
                PSDKU Nganjuk            
                    
                pt bajang12            
            
    Pilih Instansi dan periode untuk melihat kuota.


    Cari Guru (nama/NIP)
    
        
        Cari
    
    
        Pilih Guru aktif
                                        
                guruepkl · developer.guru            
            
    
        Sebelumnya
        1 / 1
        Berikutnya
        Pilihan menggunakan ID profil Guru.
    


    PIC Instansi (opsional)
    
        Tanpa PIC
            


    
        Tanggal mulai
        
    
    
        Tanggal selesai
        
    


document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, function () {
    const form = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-form&quot; , &quot;'&quot; , &quot;);
    const errorBox = document.getElementById(&quot; , &quot;'&quot; , &quot;placementReferenceError&quot; , &quot;'&quot; , &quot;);
    const studentSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;student-select&quot; , &quot;'&quot; , &quot;);
    const teacherSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-select&quot; , &quot;'&quot; , &quot;);
    const periodSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;period-select&quot; , &quot;'&quot; , &quot;);
    const instanceSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;instance-select&quot; , &quot;'&quot; , &quot;);
    const picSelect = document.getElementById(&quot; , &quot;'&quot; , &quot;pic-select&quot; , &quot;'&quot; , &quot;);
    let referenceData = {&quot;siswa&quot;:[{&quot;id&quot;:1,&quot;profile_id&quot;:1,&quot;nama_lengkap&quot;:&quot;Developer E-PKL Siswa&quot;,&quot;nisn&quot;:&quot;9999999999&quot;,&quot;nis&quot;:&quot;DEV-SISWA-001&quot;,&quot;username&quot;:&quot;developer.siswa&quot;},{&quot;id&quot;:8,&quot;profile_id&quot;:8,&quot;nama_lengkap&quot;:&quot;niha natus&quot;,&quot;nisn&quot;:&quot;0000000012&quot;,&quot;nis&quot;:&quot;00430506&quot;,&quot;username&quot;:&quot;niha&quot;},{&quot;id&quot;:4,&quot;profile_id&quot;:4,&quot;nama_lengkap&quot;:&quot;siswa testing baru&quot;,&quot;nisn&quot;:&quot;1414090988&quot;,&quot;nis&quot;:&quot;0776543&quot;,&quot;username&quot;:&quot;siswa.baru&quot;},{&quot;id&quot;:2,&quot;profile_id&quot;:2,&quot;nama_lengkap&quot;:&quot;Siswa Uji EPKL Registrasi&quot;,&quot;nisn&quot;:&quot;8888888801&quot;,&quot;nis&quot;:null,&quot;username&quot;:&quot;8888888801&quot;},{&quot;id&quot;:6,&quot;profile_id&quot;:6,&quot;nama_lengkap&quot;:&quot;siswa uji testing&quot;,&quot;nisn&quot;:&quot;5432112345&quot;,&quot;nis&quot;:&quot;00976711&quot;,&quot;username&quot;:&quot;siswauji&quot;},{&quot;id&quot;:5,&quot;profile_id&quot;:5,&quot;nama_lengkap&quot;:&quot;siswi testing baru&quot;,&quot;nisn&quot;:&quot;99996666&quot;,&quot;nis&quot;:&quot;0099876&quot;,&quot;username&quot;:&quot;siswi.baru&quot;},{&quot;id&quot;:7,&quot;profile_id&quot;:7,&quot;nama_lengkap&quot;:&quot;siswi uji&quot;,&quot;nisn&quot;:&quot;00111001&quot;,&quot;nis&quot;:&quot;12340987&quot;,&quot;username&quot;:&quot;siswi uji&quot;}],&quot;guru&quot;:[{&quot;id&quot;:1,&quot;profile_id&quot;:1,&quot;nama_lengkap&quot;:&quot;guruepkl&quot;,&quot;nip&quot;:&quot;developer.guru&quot;,&quot;username&quot;:&quot;developer.guru&quot;}],&quot;selected&quot;:{&quot;siswa&quot;:null,&quot;guru&quot;:null},&quot;pagination&quot;:{&quot;per_page&quot;:20,&quot;siswa&quot;:{&quot;current_page&quot;:1,&quot;per_page&quot;:20,&quot;total&quot;:7,&quot;last_page&quot;:1},&quot;guru&quot;:{&quot;current_page&quot;:1,&quot;per_page&quot;:20,&quot;total&quot;:1,&quot;last_page&quot;:1}},&quot;periode&quot;:[{&quot;id&quot;:1,&quot;nama&quot;:&quot;Periode Aktif&quot;,&quot;tanggal_mulai&quot;:&quot;2026-11-16&quot;,&quot;tanggal_selesai&quot;:&quot;2027-02-16&quot;,&quot;status&quot;:&quot;persiapan&quot;}],&quot;instansi&quot;:[{&quot;id&quot;:3,&quot;nama&quot;:&quot;PSDKU Nganjuk&quot;},{&quot;id&quot;:2,&quot;nama&quot;:&quot;pt bajang12&quot;}],&quot;pic_instansi&quot;:[],&quot;kuota&quot;:null};
    let referencesReady = true;
    let referenceRequest = 0;

    function updatePageLabels() {
        [&quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;guru&quot; , &quot;'&quot; , &quot;].forEach(function (key) {
            const data = referenceData.pagination?.[key];
            const label = document.getElementById(key === &quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;student-page&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;teacher-page&quot; , &quot;'&quot; , &quot;);
            const previous = document.getElementById(key === &quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;student-prev&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;teacher-prev&quot; , &quot;'&quot; , &quot;);
            const next = document.getElementById(key === &quot; , &quot;'&quot; , &quot;siswa&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;student-next&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;teacher-next&quot; , &quot;'&quot; , &quot;);
            if (!label || !data) return;
            label.textContent = data.current_page + &quot; , &quot;'&quot; , &quot; / &quot; , &quot;'&quot; , &quot; + Math.max(data.last_page, 1);
            previous.disabled = data.current_page &lt;= 1;
            next.disabled = data.current_page >= data.last_page;
        });
    }

    function setOptions(select, rows, selected, label, preserve) {
        if (!select) return;
        const oldValue = preserve ? select.value : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        select.replaceChildren(new Option(label, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));
        const visible = Array.isArray(rows) ? rows.slice() : [];
        if (selected &amp;&amp; !visible.some((row) => String(row.id) === String(selected.id))) {
            visible.unshift(selected);
        }
        visible.forEach(function (row) {
            const text = select.id === &quot; , &quot;'&quot; , &quot;student-select&quot; , &quot;'&quot; , &quot;
                ? (row.nama_lengkap || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; · &quot; , &quot;'&quot; , &quot; + (row.nisn || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)
                : (row.nama_lengkap || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; · &quot; , &quot;'&quot; , &quot; + (row.nip || &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;);
            select.add(new Option(text, row.id));
        });
        if (oldValue &amp;&amp; Array.from(select.options).some((option) => option.value === oldValue)) {
            select.value = oldValue;
        } else if (selected &amp;&amp; String(selected.id) === oldValue) {
            select.value = oldValue;
        }
    }

    function updateQuota() {
        const target = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-quota&quot; , &quot;'&quot; , &quot;);
        const quota = referenceData.kuota;
        if (!target) return;
        if (!quota) {
            target.textContent = &quot; , &quot;'&quot; , &quot;Pilih Instansi dan periode untuk melihat kuota.&quot; , &quot;'&quot; , &quot;;
        } else if (quota.tanpa_batas) {
            target.textContent = &quot; , &quot;'&quot; , &quot;Kuota tidak dibatasi; terpakai &quot; , &quot;'&quot; , &quot; + quota.terpakai + &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;;
        } else {
            target.textContent = &quot; , &quot;'&quot; , &quot;Kuota &quot; , &quot;'&quot; , &quot; + quota.kuota_siswa + &quot; , &quot;'&quot; , &quot;; terpakai &quot; , &quot;'&quot; , &quot; + quota.terpakai + &quot; , &quot;'&quot; , &quot;; sisa &quot; , &quot;'&quot; , &quot; + quota.sisa + &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;;
        }
    }

    function updateDateRange() {
        const start = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-start&quot; , &quot;'&quot; , &quot;);
        const end = document.getElementById(&quot; , &quot;'&quot; , &quot;placement-end&quot; , &quot;'&quot; , &quot;);
        let periodStart = &quot;&quot;;
        let periodEnd = &quot;&quot;;
        if (periodSelect) {
            const option = periodSelect.options[periodSelect.selectedIndex];
            periodStart = option?.dataset.start || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            periodEnd = option?.dataset.end || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        }
        start.min = periodStart;
        start.max = periodEnd;
        end.min = start.value || periodStart;
        end.max = periodEnd;
    }

    async function loadReferences(change) {
        const requestId = ++referenceRequest;
        referencesReady = false;
        errorBox.classList.add(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
        const currentStudent = studentSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const currentTeacher = teacherSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        const params = new URLSearchParams({
            page: &quot; , &quot;'&quot; , &quot;referensi-penempatan-pkl&quot; , &quot;'&quot; , &quot;,
            per_page: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;,
            siswa_search: document.getElementById(&quot; , &quot;'&quot; , &quot;student-search&quot; , &quot;'&quot; , &quot;)?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            guru_search: document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-search&quot; , &quot;'&quot; , &quot;)?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            siswa_page: String(change.siswaPage ?? referenceData.pagination?.siswa?.current_page ?? 1),
            guru_page: String(change.guruPage ?? referenceData.pagination?.guru?.current_page ?? 1),
            instansi_id: instanceSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            periode_id: periodSelect?.value || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            siswa_id: currentStudent,
            guru_id: currentTeacher
        });
        try {
            const response = await fetch(&quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot; + params.toString(), { headers: { Accept: &quot; , &quot;'&quot; , &quot;application/json&quot; , &quot;'&quot; , &quot; } });
            const body = await response.json();
            if (requestId !== referenceRequest) return;
            if (!response.ok || body.success !== true || !body.data) {
                throw new Error(body.message || &quot; , &quot;'&quot; , &quot;Referensi tidak dapat dimuat.&quot; , &quot;'&quot; , &quot;);
            }
            referenceData = body.data;
            setOptions(studentSelect, referenceData.siswa, referenceData.selected?.siswa, &quot; , &quot;'&quot; , &quot;Pilih Siswa aktif&quot; , &quot;'&quot; , &quot;, true);
            setOptions(teacherSelect, referenceData.guru, referenceData.selected?.guru, &quot; , &quot;'&quot; , &quot;Pilih Guru aktif&quot; , &quot;'&quot; , &quot;, true);
            if (picSelect) {
                const oldPic = picSelect.value;
                picSelect.replaceChildren(new Option(&quot; , &quot;'&quot; , &quot;Tanpa PIC&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));
                (referenceData.pic_instansi || []).forEach((pic) => {
                    picSelect.add(new Option(pic.nama + (pic.jabatan ? &quot; , &quot;'&quot; , &quot; · &quot; , &quot;'&quot; , &quot; + pic.jabatan : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;), pic.id));
                });
                if (Array.from(picSelect.options).some((option) => option.value === oldPic)) picSelect.value = oldPic;
            }
            updatePageLabels();
            updateQuota();
            referencesReady = true;
        } catch (error) {
            if (requestId !== referenceRequest) return;
            errorBox.textContent = error.message || &quot; , &quot;'&quot; , &quot;Referensi tidak dapat dimuat.&quot; , &quot;'&quot; , &quot;;
            errorBox.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
        }
    }

    document.getElementById(&quot; , &quot;'&quot; , &quot;student-search-button&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ siswaPage: 1 }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-search-button&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ guruPage: 1 }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;student-search&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;, (event) => {
        if (event.key === &quot; , &quot;'&quot; , &quot;Enter&quot; , &quot;'&quot; , &quot;) { event.preventDefault(); loadReferences({ siswaPage: 1 }); }
    });
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-search&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;, (event) => {
        if (event.key === &quot; , &quot;'&quot; , &quot;Enter&quot; , &quot;'&quot; , &quot;) { event.preventDefault(); loadReferences({ guruPage: 1 }); }
    });
    document.getElementById(&quot; , &quot;'&quot; , &quot;student-prev&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ siswaPage: Math.max(1, (referenceData.pagination?.siswa?.current_page || 1) - 1) }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;student-next&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ siswaPage: (referenceData.pagination?.siswa?.current_page || 1) + 1 }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-prev&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ guruPage: Math.max(1, (referenceData.pagination?.guru?.current_page || 1) - 1) }));
    document.getElementById(&quot; , &quot;'&quot; , &quot;teacher-next&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, () => loadReferences({ guruPage: (referenceData.pagination?.guru?.current_page || 1) + 1 }));
    instanceSelect?.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, () => loadReferences({}));
    periodSelect?.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, () => { updateDateRange(); loadReferences({}); });
    document.getElementById(&quot; , &quot;'&quot; , &quot;placement-start&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, updateDateRange);
    updatePageLabels();
    updateQuota();
    updateDateRange();
    form?.addEventListener(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function (event) {
        if (!referencesReady) {
            event.preventDefault();
            errorBox.textContent = &quot; , &quot;'&quot; , &quot;Form tidak dapat dikirim sebelum data referensi berhasil dimuat.&quot; , &quot;'&quot; , &quot;;
            errorBox.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
        }
    });
});

            
                Batal
                
                    Simpan Penempatan
                
            
        
    &quot;))]</value>
      <webElementGuid>e891835c-49e0-44e2-8fa2-0320f82c1d67</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
